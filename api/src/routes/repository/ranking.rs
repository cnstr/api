use crate::db::prisma;

use crate::prisma::repository;
use prisma_client_rust::Direction;
use serde_json::{json, to_string_pretty};
use tide::{
	prelude::Deserialize,
	Request, Response, Result,
	StatusCode::{BadRequest, UnprocessableEntity},
};
use tokio::runtime::Builder;

#[derive(Deserialize)]
struct Query {
	rank: Option<String>,
}

pub async fn repository_ranking(req: Request<()>) -> Result {
	let query = match req.query::<Query>() {
		Ok(query) => {
			let rank = match query.rank {
				Some(q) => {
					let match_q = match q.as_str() {
						"1" => q,
						"2" => q,
						"3" => q,
						"4" => q,
						"5" => q,
						"*" => q,
						_ => {
							return Ok(Response::builder(BadRequest)
								.body(
									to_string_pretty(&json!({
										"message": "400 Bad Request",
										"error": "Query parameter \'rank\' must be 1, 2, 3, 4, 5, or *",
										"date": chrono::Utc::now().to_rfc3339(),
									}))
									.unwrap(),
								)
								.build());
						}
					};

					match_q
				}
				None => {
					return Ok(Response::builder(BadRequest)
						.body(
							to_string_pretty(&json!({
								"message": "400 Bad Request",
								"error": "Missing query parameter: \'q\'",
								"date": chrono::Utc::now().to_rfc3339(),
							}))
							.unwrap(),
						)
						.build());
				}
			};

			rank
		}

		Err(err) => {
			println!("Error: {}", err);
			return Ok(Response::builder(UnprocessableEntity)
				.body(
					to_string_pretty(&json!({
						"message": "422 Unprocessable Entity",
						"error": "Malformed query parameters",
						"date": chrono::Utc::now().to_rfc3339(),
					}))
					.unwrap(),
				)
				.build());
		}
	};

	let repositories = Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async move {
			return match query.as_str() {
				"*" => prisma()
					.await
					.repository()
					.find_many(vec![repository::is_pruned::equals(false)])
					.order_by(repository::tier::order(Direction::Asc))
					.with(repository::origin::fetch())
					.exec()
					.await
					.unwrap(),
				_ => prisma()
					.await
					.repository()
					.find_many(vec![
						repository::tier::equals(query.parse::<i32>().unwrap()),
						repository::is_pruned::equals(false),
					])
					.order_by(repository::tier::order(Direction::Asc))
					.with(repository::origin::fetch())
					.exec()
					.await
					.unwrap(),
			};
		});

	return Ok(to_string_pretty(&json!({
		"message": "200 Successful",
		"date": chrono::Utc::now().to_rfc3339(),
		"count": repositories.len(),
		"data": repositories,
	}))
	.unwrap()
	.into());
}
