use crate::utility::api_respond;
use db::{create_prisma, create_typesense_client};
use serde_json::json;
use std::{future::Future, pin::Pin};
use tide::{
	security::{CorsMiddleware, Origin},
	utils::After,
	Next, Request, Response, Result,
};
use tokio::io::Error;

mod db;
pub mod prisma;
mod routes;
pub mod utility;

#[warn(clippy::all)]
#[warn(clippy::correctness)]
#[warn(clippy::suspicious)]
#[warn(clippy::pedantic)]
#[warn(clippy::style)]
#[warn(clippy::complexity)]
#[warn(clippy::perf)]
#[tokio::main]
async fn main() -> Result<()> {
	create_prisma().await;
	create_typesense_client();

	let mut app = tide::new();
	let cors = CorsMiddleware::new().allow_origin(Origin::from("*"));

	app.with(cors);
	app.with(response_time);
	app.with(After(|res: Response| async {
		if let Some(err) = res.downcast_error::<Error>() {
			println!("Error: {}", err);
			return api_respond(500, json!({}));
		}

		Ok(res)
	}));

	app.at("/").get(routes::index);
	app.at("/healthz").get(routes::health);
	app.at("/openapi.json").get(routes::openapi_json);
	app.at("/openapi.yaml").get(routes::openapi_yaml);

	app.at("/jailbreak/package").nest({
		let mut nest = tide::new();
		nest.at("/search").get(routes::package_search);
		nest.at("/multi").get(routes::package_multi_lookup);
		nest.at("/:package").get(routes::package_lookup);
		nest
	});

	app.at("/jailbreak/repository").nest({
		let mut nest = tide::new();
		nest.at("/search").get(routes::repository_search);
		nest.at("/ranking").get(routes::repository_ranking);
		nest.at("/safety").get(routes::repository_safety);
		nest.at("/:repository").get(routes::repository_lookup);
		nest.at("/:repository/packages")
			.get(routes::repository_packages);
		nest
	});

	app.at("*").all(routes::not_found);
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
