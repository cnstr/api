use crate::{
	helpers::{pg_client, responses, row_to_value},
	utility::{api_endpoint, merge_json},
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::{json, Value};

pub async fn packages(id: Path<String>) -> impl IntoResponse {
	let repository = match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
                        SELECT * FROM repository
                        WHERE
                            visible = true
                            AND id = $1
                        LIMIT 1
                    ",
					&[&id.to_string()],
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

	if repository.len() == 0 {
		return responses::error(StatusCode::NOT_FOUND, "Repository not found");
	}

	let row = &repository[0];
	let id: String = row.get("id");

	let packages = match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
                        SELECT * FROM package
                        WHERE
                            visible = true
                            AND repository_id = $1
                    ",
					&[&id.to_string()],
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

	responses::data_with_count(
		StatusCode::OK,
		packages
			.iter()
			.map(|row| {
				let package_id: String = row.get("package_id");

				merge_json(
					row_to_value(&row),
					json!({
						"refs": {
							"meta": format!("{}/jailbreak/package/{}", api_endpoint(), package_id),
							"repo": format!("{}/jailbreak/repository/{}", api_endpoint(), id),
						}
					}),
				)
			})
			.collect::<Vec<Value>>(),
		packages.len(),
	)
}

pub async fn packages_healthy() -> bool {
	let repository = match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
                        SELECT * FROM repository
                        WHERE
                            visible = true
                            AND id = 'havoc'
                        LIMIT 1
                    ",
					&[],
				)
				.await
			{
				Ok(rows) => rows,
				Err(_) => return false,
			}
		}
		Err(_) => return false,
	};

	if repository.len() == 0 {
		return false;
	}

	match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
                        SELECT * FROM package
                        WHERE
                            visible = true
                            AND repository_id = 'havoc'
                        LIMIT 1000
                    ",
					&[],
				)
				.await
			{
				Ok(_) => true,
				Err(_) => false,
			}
		}
		Err(_) => false,
	}
}
