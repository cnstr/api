mod db;
pub mod prisma;
mod routes;

use std::{future::Future, pin::Pin};

use tide::{
	security::{CorsMiddleware, Origin},
	Next, Request, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
	let mut app = tide::new();
	let cors = CorsMiddleware::new().allow_origin(Origin::from("*"));

	app.with(cors);
	app.with(response_time);

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
