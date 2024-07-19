use crate::{
	helpers::{pg_client, responses, row_to_value},
	utility::{api_endpoint, merge_json},
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct MultiLookupParams {
	ids: Option<String>,
	priority: Option<String>,
}

pub async fn multi_lookup(query: Query<MultiLookupParams>) -> impl IntoResponse {
	let ids = match &query.ids {
		Some(ids) => {
			let ids: Vec<String> = ids.split(',').map(|id| id.to_string()).collect();
			ids
		}
		None => {
			return responses::error(StatusCode::BAD_REQUEST, "Missing query parameter: \'ids\'");
		}
	};

	let priority = match &query.priority {
		Some(priority) => priority,
		None => "default",
	};

	let mut packages = match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
                        SELECT package.*, repository.bootstrap
                        FROM package
                        LEFT JOIN
                            repository ON
                            package.repository_id = repository.id
                        WHERE
                            package.visible = true
                            AND latest_version = true
                            AND package_id = ANY($1)
                        ORDER BY
                            quality ASC
                    ",
					&[&ids],
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
		return responses::error(StatusCode::NOT_FOUND, "Packages not found");
	}

	let mut ids: Vec<String> = packages
		.iter()
		.map(|package| package.get("package_id"))
		.collect();

	packages.sort_by(|a, b| {
		// If the priority is bootstrap, prioritize package.repository.is_bootstrap
		if priority == "bootstrap" {
			let a_bootstrap: bool = a.get("bootstrap");
			let b_bootstrap: bool = b.get("bootstrap");

			if a_bootstrap && !b_bootstrap {
				return std::cmp::Ordering::Less;
			} else if !a_bootstrap && b_bootstrap {
				return std::cmp::Ordering::Greater;
			}
		}

		let a_quality: i32 = a.get("quality");
		let b_quality: i32 = b.get("quality");

		// If the priority is default, prioritize package.repository_tier
		if a_quality < b_quality {
			return std::cmp::Ordering::Less;
		} else if a_quality > b_quality {
			return std::cmp::Ordering::Greater;
		}

		return std::cmp::Ordering::Equal;
	});

	responses::data_with_count(
		StatusCode::OK,
		packages
			.iter()
			.filter(|package| {
				let package_id: String = package.get("package_id");
				if ids.contains(&package_id) {
					ids.retain(|id| id != &package_id);
					return true;
				}
				return false;
			})
			.map(|package| {
				let repository_id: String = package.get("repository_id");
				return merge_json(
					row_to_value(package),
					json!({
						"refs": {
							"repo": format!("{}/jailbreak/repository/{}", api_endpoint(), repository_id)
						}
					}),
				);
			})
			.collect::<Vec<Value>>(),
		packages.len(),
	)
}

pub async fn multi_lookup_healthy() -> bool {
	match pg_client().await {
		Ok(pg_client) => {
			let rows = pg_client
				.query(
					"
                        SELECT package.*, repository.bootstrap
                        FROM package
                        LEFT JOIN
                            repository ON
                            package.repository_id = repository.id
                        WHERE
                            package.visible = true
                            AND latest_version = true
                            AND package_id = ANY($1)
                        ORDER BY
                            quality ASC
                    ",
					&[&vec!["ws.hbang.common", "com.amywhile.aemulo"]],
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
