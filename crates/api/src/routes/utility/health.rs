use crate::{
	db::{prisma, typesense},
	utility::{handle_async, http_respond},
};
use prisma_client_rust::Raw;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
/// Calculated by checking the health of Typesense and Postgres
pub async fn health(_req: Request<()>) -> Result {
	let (typesense_healthy, postgres_healthy) = handle_async(async move {
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

		(typesense_healthy, postgres_healthy)
	});

	let status_code = match typesense_healthy && postgres_healthy {
		true => 200,
		false => 500,
	};

	http_respond(
		status_code,
		json!({
			"healthy": typesense_healthy && postgres_healthy,
			"typesense_healthy": typesense_healthy,
			"postgres_healthy": postgres_healthy,
		}),
	)
}
