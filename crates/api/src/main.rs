use crate::utility::api_respond;
use anyhow::Error;
use sentry::{init, ClientOptions};
use serde_json::json;
use std::{future::Future, pin::Pin};
use tide::{
	security::{CorsMiddleware, Origin},
	utils::After,
	Next, Request, Response, Result,
};
use utility::{create_prisma_client, create_typesense_client, handle_error};

mod prisma;
mod routes;
mod utility;

#[warn(clippy::all)]
#[warn(clippy::correctness)]
#[warn(clippy::suspicious)]
#[warn(clippy::pedantic)]
#[warn(clippy::style)]
#[warn(clippy::complexity)]
#[warn(clippy::perf)]
#[tokio::main]
async fn main() -> Result<()> {
	let _guard = init((
		env!("CANISTER_SENTRY_DSN"),
		ClientOptions {
			release: Some(env!("VERGEN_BUILD_SEMVER").into()),
			traces_sample_rate: 0.5,
			..Default::default()
		},
	));

	create_prisma_client().await;
	create_typesense_client();

	let mut app = tide::new();
	let cors = CorsMiddleware::new().allow_origin(Origin::from("*"));

	app.with(cors);
	app.with(response_time);
	app.with(After(|res: Response| async {
		if let Some(err) = res.downcast_error::<Error>() {
			handle_error(err);
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
	Ok(())
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
		Ok(res)
	})
}
