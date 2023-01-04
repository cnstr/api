mod db;
pub mod prisma;
mod routes;
pub mod utility;

use serde_json::json;
use std::{future::Future, pin::Pin};
use tokio::io::Error;

use db::{create_elastic, create_prisma};
use tide::{
	security::{CorsMiddleware, Origin},
	utils::After,
	Next, Request, Response, Result,
	StatusCode::InternalServerError,
};
use utility::json_respond;

#[tokio::main]
async fn main() -> Result<()> {
	create_prisma().await;
	create_elastic().await;

	let mut app = tide::new();
	let cors = CorsMiddleware::new().allow_origin(Origin::from("*"));

	app.with(cors);
	app.with(response_time);
	app.with(After(|mut res: Response| async {
		if let Some(err) = res.downcast_error::<Error>() {
			println!("Error: {}", err);
			res = json_respond(
				InternalServerError,
				json!({
					"message": "500 Internal Server Error",
					"date": chrono::Utc::now().to_rfc3339(),
				}),
			);
		}

		Ok(res)
	}));

	app.at("/v2").nest({
		let mut api = tide::new();

		api.at("/").get(routes::index);
		api.at("/healthz").get(routes::health);
		api.at("/openapi.json").get(routes::openapi_json);
		api.at("/openapi.yaml").get(routes::openapi_yaml);

		api.at("/jailbreak/package").nest({
			let mut nest = tide::new();
			nest.at("/search").get(routes::package_search);
			nest.at("/multi").get(routes::package_multi_lookup);
			nest.at("/:package").get(routes::package_lookup);
			nest
		});

		api.at("/jailbreak/repository").nest({
			let mut nest = tide::new();
			nest.at("/search").get(routes::repository_search);
			nest.at("/ranking").get(routes::repository_ranking);
			nest.at("/safety").get(routes::repository_safety);
			nest.at("/:repository").get(routes::repository_lookup);
			nest.at("/:repository/packages")
				.get(routes::repository_packages);
			nest
		});

		api
	});

	app.listen("0.0.0.0:3000").await?;
	return Ok(());
}

fn response_time<'a>(
	req: Request<()>,
	next: Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
	let start = std::time::Instant::now();
	Box::pin(async move {
		let mut res = next.run(req).await;
		let elapsed = start.elapsed().as_millis();

		res.insert_header("X-Response-Time", elapsed.to_string());
		return Ok(res);
	})
}
