use crate::{
	helpers::{pg_client, responses, row_to_value},
	utility::{api_endpoint, handle_error, merge_json},
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct RankingParams {
	rank: Option<String>,
}

pub async fn ranking(query: Query<RankingParams>) -> impl IntoResponse {
	let rank = match &query.rank {
		Some(q) => {
			let match_q = match q.as_str() {
				"1" => q,
				"2" => q,
				"3" => q,
				"4" => q,
				"5" => q,
				"*" => q,
				_ => {
					return responses::error(
						StatusCode::BAD_REQUEST,
						"Query parameter \'rank\' must be 1, 2, 3, 4, 5, or *",
					)
				}
			};

			match_q
		}

		None => {
			return responses::error(StatusCode::BAD_REQUEST, "Missing query parameter: \'rank\'")
		}
	};

	let lookup = match rank.as_str() {
		"*" => match pg_client().await {
			Ok(pg_client) => {
				pg_client
					.query(
						"
                            SELECT * FROM repository
                            WHERE visible = true
                            ORDER BY quality ASC
                        ",
						&[],
					)
					.await
			}
			Err(e) => {
				eprintln!("[db] Failed to query database: {}", e);
				return responses::error(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to query database",
				);
			}
		},

		_ => match pg_client().await {
			Ok(pg_client) => {
				let rank = rank.parse::<i32>().unwrap_or_else(|err| {
					handle_error(&err.into());
					1
				});

				pg_client
					.query(
						"
                            SELECT * FROM repository
                            WHERE visible = true AND quality = $1
                            ORDER BY quality ASC
                        ",
						&[&rank],
					)
					.await
			}
			Err(e) => {
				eprintln!("[db] Failed to query database: {}", e);
				return responses::error(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to query database",
				);
			}
		},
	};

	let repositories = match lookup {
		Ok(rows) => rows,
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
		repositories
			.iter()
			.map(|row| {
				let id: String = row.get("id");

				return merge_json(
					row_to_value(row),
					json!({
						"refs": {
							"meta": format!("{}/jailbreak/repository/{}", api_endpoint(), id),
							"packages": format!("{}/jailbreak/repository/{}/packages", api_endpoint(), id),
						}
					}),
				);
			})
			.collect::<Vec<Value>>(),
		repositories.len(),
	)
}
