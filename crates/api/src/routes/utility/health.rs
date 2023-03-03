use crate::{
	routes,
	utility::{handle_async, http_respond, prisma, typesense},
};
use prisma_client_rust::Raw;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tide::{Request, Result};

/// Represents the /health response from Typesense
#[derive(Serialize, Deserialize)]
struct TypesenseHealth {
	ok: bool,
}

/// Represents the output of 'SELECT 1;' from Postgres
#[derive(Serialize, Deserialize)]
struct PostgresHealth {
	#[serde(rename = "?column?")]
	column: u16,
}

/// Returns the health of the API
/// Calculated by checking the health of most of the routes that the API advertises
pub async fn health(_req: Request<()>) -> Result {
	let (service_healthy, service_data) = service_healthy().await;
	let (package_healthy, package_data) = package_healthy().await;
	let (repository_healthy, repository_data) = repository_healthy().await;

	let healthy = service_healthy && package_healthy && repository_healthy;

	let data = json!({
		"healthy": healthy,
		"service_data": service_data,
		"route_data": {
			"package": package_data,
			"repository": repository_data,
		},
	});

	let status_code = match healthy {
		true => 200,
		false => 500,
	};

	http_respond(status_code, data)
}

async fn service_healthy() -> (bool, Value) {
	return handle_async(async move {
		let typesense_healthy = match typesense().get("/health").await {
			Ok(mut response) => {
				let health: TypesenseHealth = match response.body_json().await {
					Ok(health) => health,
					Err(err) => {
						println!("Failed to serialize Typesense health response: {}", err);
						TypesenseHealth { ok: false }
					}
				};

				health.ok
			}
			Err(err) => {
				println!("Typesense health check failed: {}", err);
				false
			}
		};

		let postgres_healthy = match prisma()
			._query_raw::<PostgresHealth>(Raw::new("SELECT 1;", vec![]))
			.exec()
			.await
		{
			Ok(health) => {
				if health.len() == 1 {
					health[0].column == 1
				} else {
					false
				}
			}
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
	});
}

async fn package_healthy() -> (bool, Value) {
	let lookup_healthy = routes::package_lookup_healthy().await;
	let multi_lookup_healthy = routes::package_multi_lookup_healthy().await;
	let search_healthy = routes::package_search_healthy().await;

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
	let lookup_healthy = routes::repository_lookup_healthy().await;
	let packages_healthy = routes::repository_packages_healthy().await;
	let safety_healthy = routes::repository_safety_healthy().await;
	let search_healthy = routes::repository_search_healthy().await;

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
