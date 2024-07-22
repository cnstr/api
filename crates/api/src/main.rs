use crate::{helpers::create_db, utility::load_runtime_config};
use axum::{
	http::{HeaderValue, Request, StatusCode},
	middleware::{self, Next},
	response::Response,
	routing::{get, post},
	Json, Router,
};
use chrono::Utc;
use sentry::{capture_message, init, integrations::anyhow::capture_anyhow, ClientOptions, Level};
use serde_json::json;
use services::create_ts;
use std::{net::SocketAddr, process::exit, sync::OnceLock};

mod helpers;
mod routes;
mod services;
mod types;
mod utility;

#[warn(clippy::all)]
#[warn(clippy::correctness)]
#[warn(clippy::suspicious)]
#[warn(clippy::pedantic)]
#[warn(clippy::style)]
#[warn(clippy::complexity)]
#[warn(clippy::perf)]

static POD_NAME: OnceLock<String> = OnceLock::new();

/// Main entry point for the HTTP server
/// All route handlers run in a tokio context
#[tokio::main]
async fn main() {
	let config = load_runtime_config();

	let _guard = init((
		config.sentry_dsn.as_str(),
		ClientOptions {
			release: Some(env!("VERGEN_BUILD_SEMVER").into()),
			traces_sample_rate: 0.5,
			..Default::default()
		},
	));

	if let Err(e) = create_db().await {
		capture_anyhow(&e);
		eprintln!("[indexer] failed to connect to postgres: {}", e);
		exit(1);
	}

	if let Err(e) = create_ts().connect().await {
		capture_anyhow(&e);
		eprintln!("[indexer] failed to connect to typesense: {}", e);
		exit(1);
	}

	let app = Router::new()
		.route("/v2/", get(routes::info::landing_page))
		.route("/v2/healthz", get(routes::info::health_check))
		.route("/v2/openapi.json", get(routes::info::openapi_json))
		.route("/v2/openapi.yaml", get(routes::info::openapi_yaml))
		.route(
			"/v2/jailbreak/download/ingest",
			post(routes::download::ingest),
		)
		.route("/v2/jailbreak/package/search", get(routes::package::search))
		.route(
			"/v2/jailbreak/package/:package",
			get(routes::package::lookup),
		)
		.route(
			"/v2/jailbreak/package/multi",
			get(routes::package::multi_lookup),
		)
		.route(
			"/v2/jailbreak/repository/ranking",
			get(routes::repository::ranking),
		)
		.route(
			"/v2/jailbreak/repository/safety",
			get(routes::repository::safety),
		)
		.route(
			"/v2/jailbreak/repository/search",
			get(routes::repository::search),
		)
		.route(
			"/v2/jailbreak/repository/:repository",
			get(routes::repository::lookup),
		)
		.route(
			"/v2/jailbreak/repository/:repository/packages",
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

	// Also add the X-Served-By header and X-Request-ID (TODO)
	let pod_name = POD_NAME.get_or_init(|| {
		let pod_name = std::env::var("POD_NAME").unwrap_or_else(|_| "unknown".to_string());
		pod_name
	});

	headers.insert(
		"X-Served-By",
		HeaderValue::from_str(pod_name).unwrap_or(HeaderValue::from_static("unknown")),
	);
	response
}
