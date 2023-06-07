use axum::{
	http::{HeaderValue, Request, StatusCode},
	middleware::{self, Next},
	response::Response,
	routing::{get, post},
	Json, Router,
};
use chrono::Utc;
use sentry::{capture_message, init, ClientOptions, Level};
use serde_json::json;
use std::{net::SocketAddr, process::exit};

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

	let app = Router::new()
		.route("/", get(routes::info::landing_page))
		.route("/healthz", get(routes::info::health_check))
		.route("/openapi.json", get(routes::info::openapi_json))
		.route("/openapi.yaml", get(routes::info::openapi_yaml))
		.route("/jailbreak/download/ingest", post(routes::download::ingest))
		.route("/jailbreak/package/search", get(routes::package::search))
		.route("/jailbreak/package/:package", get(routes::package::lookup))
		.route(
			"/jailbreak/package/multi",
			get(routes::package::multi_lookup),
		)
		.route(
			"/jailbreak/repository/ranking",
			get(routes::repository::ranking),
		)
		.route(
			"/jailbreak/repository/safety",
			get(routes::repository::safety),
		)
		.route(
			"/jailbreak/repository/search",
			get(routes::repository::search),
		)
		.route(
			"/jailbreak/repository/:repository",
			get(routes::repository::lookup),
		)
		.route(
			"/jailbreak/repository/:repository/packages",
			get(routes::repository::packages),
		)
		.layer(middleware::from_fn(cors_middleware))
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
			capture_message("failed to bind http port", Level::Fatal);
			println!("panic: failed to bind http port - {err}");
			exit(1);
		});
}

async fn cors_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
	let mut response = next.run(request).await;

	let headers = response.headers_mut();

	headers.insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));
	headers.insert(
		"Access-Control-Allow-Methods",
		HeaderValue::from_static("GET, POST, OPTIONS"),
	);
	headers.insert(
		"Access-Control-Allow-Headers",
		HeaderValue::from_static("Content-Type, *"),
	);

	response
}
