use crate::{
	helpers::{pg_client, responses, row_to_value},
	utility::{api_endpoint, merge_json, page_links},
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct SearchParams {
	q: Option<String>,
	limit: Option<u8>,
	page: Option<u8>,
}

pub async fn search(query: Query<SearchParams>) -> impl IntoResponse {
	let q = match &query.q {
		Some(q) => {
			if q.len() < 2 {
				return responses::error(
					StatusCode::BAD_REQUEST,
					"Query parameter \'q\' must be at least 2 characters",
				);
			}

			q
		}

		None => {
			return responses::error(StatusCode::BAD_REQUEST, "Missing query parameter: \'q\'");
		}
	};

	let page = match query.page {
		Some(page) => {
			if page < 1 {
				return responses::error(
					StatusCode::BAD_REQUEST,
					"Query parameter \'page\' must be greater than 0",
				);
			}

			page
		}

		None => 1,
	};

	let limit = match query.limit {
		Some(limit) => {
			if !(1..=250).contains(&limit) {
				return responses::error(
					StatusCode::BAD_REQUEST,
					"Query parameter \'limit\' must be between 1 and 250",
				);
			}

			limit
		}

		None => 100,
	};

	let repositories = match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
                        SELECT *, ts_rank(
	                        search_vector,
							plainto_tsquery('simple', $1)
                        ) AS rank
                        FROM repository
                        WHERE
                            visible = true
                            AND search_vector @@ plainto_tsquery('simple', $1)
                        ORDER BY
                        	rank DESC,
                         	quality ASC
                        LIMIT $2 OFFSET $3
                    ",
					&[
						&q.to_string(),
						&(limit as i64),
						&(((page - 1) * limit) as i64),
					],
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

	let next = repositories.len() == limit as usize;
	let (prev_page, next_page) = page_links("/jailbreak/repository/search", page, next);

	responses::data_with_count_and_refs(
		StatusCode::OK,
		&repositories
			.iter()
			.map(|row| {
				let id: String = row.get("id");
				merge_json(
					row_to_value(row),
					json!({
						"refs": {
							"meta": format!("{}/jailbreak/repository/{}", api_endpoint(), id),
							"packages": format!("{}/jailbreak/repository/{}/packages", api_endpoint(), id)
						}
					}),
				)
			})
			.collect::<Vec<Value>>(),
		repositories.len(),
		json!({
			"nextPage": next_page,
			"previousPage": prev_page,
		}),
	)
}

pub async fn search_healthy() -> bool {
	match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
						SELECT *, ts_rank(
							search_vector,
							plainto_tsquery('simple', 'havoc')
                        ) AS rank
                        FROM repository
                        WHERE
                            visible = true
                            AND search_vector @@ plainto_tsquery('simple', 'havoc')
                        ORDER BY
                        	rank DESC,
                         	quality ASC
                        LIMIT 1 OFFSET 0
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
