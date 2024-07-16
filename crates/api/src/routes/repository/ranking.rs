use crate::{
	helpers::{clients, responses},
	prisma::repository,
	utility::{api_endpoint, handle_error, merge_json},
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use prisma_client_rust::Direction;
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
		"*" => {
			clients::prisma(|prisma| {
				prisma
					.repository()
					.find_many(vec![repository::is_pruned::equals(false)])
					.order_by(repository::tier::order(Direction::Asc))
					.with(repository::origin::fetch())
					.exec()
			})
			.await
		}

		_ => {
			clients::prisma(|prisma| {
				prisma
					.repository()
					.find_many(vec![
						repository::tier::equals(rank.parse::<i32>().unwrap_or_else(|err| {
							// TODO: Report Error Correctly
							handle_error(&err.into());
							1
						})),
						repository::is_pruned::equals(false),
					])
					.order_by(repository::tier::order(Direction::Asc))
					.with(repository::origin::fetch())
					.exec()
			})
			.await
		}
	};

	let repositories = match lookup {
		Ok(repositories) => repositories,
		Err(_) => {
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
			.map(|repository| {
				let slug = repository.slug.clone();

				return merge_json(
					repository,
					json!({
						"refs": {
							"meta": format!("{}/jailbreak/repository/{}", api_endpoint(), slug),
							"packages": format!("{}/jailbreak/repository/{}/packages", api_endpoint(), slug),
						}
					}),
				);
			})
			.collect::<Vec<Value>>(),
		repositories.len(),
	)
}
