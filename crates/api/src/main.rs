use axum::{
	http::StatusCode,
	routing::{get, post},
	Json, Router,
};
use chrono::Utc;
use sentry::{init, ClientOptions};
use serde_json::json;
use std::{net::SocketAddr, process::exit};
use tower_http::cors::{Any, CorsLayer};
use utility::{create_prisma_client, create_typesense_client};

mod helpers;
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

/// Main entry point for the HTTP server
/// All route handlers run in a tokio context
#[tokio::main]
async fn main() {
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

	let cors = CorsLayer::new().allow_origin(Any);

	let app = Router::new()
		.layer(cors)
		.route("/", get(routes::info::landing_page))
		.route("/healthz", get(routes::info::health_check))
		.route("/openapi.json", get(routes::info::openapi_json))
		.route("/openapi.yaml", get(routes::info::openapi_yaml))
		.route("/jailbreak/download/ingest", post(routes::download::ingest))
		.fallback(|| async {
			(
				StatusCode::NOT_FOUND,
				Json(json!({
					"status": "404 Not Found",
					"date": Utc::now().to_rfc3339()
				})),
			)
		});

	// TODO: Error Handler?
	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

	println!("http: listening on {addr}");
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap_or_else(|err| {
			println!("http: failed to bind - {err}");
			exit(1);
		});

	// app.at("/jailbreak/package").nest({
	// 	let mut nest = tide::new();
	// 	nest.at("/search").get(routes::package_search);
	// 	nest.at("/multi").get(routes::package_multi_lookup);
	// 	nest.at("/:package").get(routes::package_lookup);
	// 	nest
	// });
	//
	// app.at("/jailbreak/repository").nest({
	// 	let mut nest = tide::new();
	// 	nest.at("/search").get(routes::repository_search);
	// 	nest.at("/ranking").get(routes::repository_ranking);
	// 	nest.at("/safety").get(routes::repository_safety);
	// 	nest.at("/:repository").get(routes::repository_lookup);
	// 	nest.at("/:repository/packages")
	// 		.get(routes::repository_packages);
	// 	nest
	// });
	//
	// app.at("/jailbreak/download").nest({
	// 	let mut nest = tide::new();
	// 	nest.at("/ingest").post(routes::download_ingest);
	// 	nest
	// });
	//
	// app.at("*").all(routes::not_found);
	// app.listen("0.0.0.0:3000").await?;
	// Ok(())
}
