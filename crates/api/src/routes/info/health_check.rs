use crate::{
	helpers::{clients, responses},
	routes,
	utility::prisma,
};
use axum::{http::StatusCode, response::IntoResponse};
use prisma_client_rust::Raw;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Represents the /health response from Typesense
#[derive(Serialize, Deserialize)]
struct TypesenseHealth {
	ok: bool,
}

/// Represents the output of 'SELECT version();' from Postgres
#[derive(Serialize, Deserialize)]
struct PostgresHealth {
	version: Option<String>,
}

pub async fn health_check() -> impl IntoResponse {
	let (service_healthy, service_data) = service_healthy().await;
	let (package_healthy, package_data) = package_healthy().await;
	let (repository_healthy, repository_data) = repository_healthy().await;
	let (download_healthy, download_data) = download_healthy().await;

	let healthy = service_healthy && package_healthy && repository_healthy && download_healthy;

	let status_code = match healthy {
		true => StatusCode::OK,
		false => StatusCode::INTERNAL_SERVER_ERROR,
	};

	responses::data(
		status_code,
		json!({
			"healthy": healthy,
			"service_data": service_data,
			"route_data": {
				"package": package_data,
				"repository": repository_data,
				"download": download_data,
			},
		}),
	)
}

async fn service_healthy() -> (bool, Value) {
	let typesense_healthy = match clients::typesense::<TypesenseHealth>({}, "health").await {
		Ok(data) => data.ok,
		Err(err) => {
			println!("Typesense health check failed: {}", err);
			false
		}
	};

	let postgres_healthy = match prisma()
		._query_raw::<PostgresHealth>(Raw::new("SELECT version();", vec![]))
		.exec()
		.await
	{
		Ok(health) => health.len() > 0,
		Err(err) => {
			println!("Postgres health check failed: {}", err);
			false
		}
	};

	let healthy = typesense_healthy && postgres_healthy;
	let value = json!({
		"healthy": healthy,
		"typesense_healthy": typesense_healthy,
		"postgres_healthy": postgres_healthy,
	});

	(healthy, value)
}

async fn package_healthy() -> (bool, Value) {
	let lookup_healthy = routes::package::lookup_healthy().await;
	let multi_lookup_healthy = routes::package::multi_lookup_healthy().await;
	let search_healthy = routes::package::search_healthy().await;

	let healthy = lookup_healthy && multi_lookup_healthy && search_healthy;
	let value = json!({
		"healthy": healthy,
		"lookup_healthy": lookup_healthy,
		"multi_lookup_healthy": multi_lookup_healthy,
		"search_healthy": search_healthy,
	});

	(healthy, value)
}

async fn repository_healthy() -> (bool, Value) {
	let lookup_healthy = routes::repository::lookup_healthy().await;
	let packages_healthy = routes::repository::packages_healthy().await;
	let safety_healthy = routes::repository::safety_healthy().await;
	let search_healthy = routes::repository::search_healthy().await;

	let healthy = lookup_healthy && packages_healthy && safety_healthy && search_healthy;
	let value = json!({
		"healthy": healthy,
		"lookup_healthy": lookup_healthy,
		"packages_healthy": packages_healthy,
		"safety_healthy": safety_healthy,
		"search_healthy": search_healthy,
	});

	(healthy, value)
}

async fn download_healthy() -> (bool, Value) {
	let ingest_healthy = routes::download::ingest_healthy().await;

	let healthy = ingest_healthy;
	let value = json!({
		"healthy": healthy,
		"ingest_healthy": ingest_healthy,
	});

	(healthy, value)
}
