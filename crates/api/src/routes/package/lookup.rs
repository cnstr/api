use crate::{
	helpers::{pg_client, responses, row_to_value},
	utility::{api_endpoint, merge_json},
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::{json, Value};

pub async fn lookup(package: Path<String>) -> impl IntoResponse {
	let packages = match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
                        SELECT * FROM package
                        WHERE
                            visible = true
                            AND package_id = $1
                        ORDER BY
                            latest_version DESC,
                            quality ASC
                    ",
					&[&package.to_string()],
				)
				.await
			{
				Ok(rows) => rows,
				Err(e) => {
					eprintln!("[db] Failed to query database: {}", e);
					return responses::error(
						StatusCode::INTERNAL_SERVER_ERROR,
						"Failed to query database",
					);
				}
			}
		}
		Err(e) => {
			eprintln!("[db] Failed to query database: {}", e);
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to query database",
			);
		}
	};

	if packages.is_empty() {
		return responses::error(StatusCode::NOT_FOUND, "Package not found");
	}

	responses::data_with_count(
		StatusCode::OK,
		packages
			.iter()
			.map(|row| {
				let id: String = row.get("repository_id");
				return merge_json(
					row_to_value(row),
					json!({
						"refs": {
							"repo": format!("{}/jailbreak/repository/{}", api_endpoint(), id)
						}
					}),
				);
			})
			.collect::<Vec<Value>>(),
		packages.len(),
	)
}

pub async fn lookup_healthy() -> bool {
	match pg_client().await {
		Ok(pg_client) => {
			let rows = pg_client
				.query(
					"
                        SELECT * FROM package
                        WHERE
                            visible = true
                            AND package_id = 'ws.hbang.common'
                        ORDER BY
                            latest_version DESC,
                            quality ASC
                    ",
					&[],
				)
				.await;

			match rows {
				Ok(_) => true,
				Err(_) => false,
			}
		}
		Err(_) => false,
	}
}
