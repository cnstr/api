use crate::utility::{merge_json, tokio_run};
use crate::{db::prisma, utility::json_respond};

use crate::prisma::repository;
use prisma_client_rust::Direction;
use serde_json::{json, Value};
use tide::{
	prelude::Deserialize,
	Request, Result,
	StatusCode::{BadRequest, Ok as OK, UnprocessableEntity},
};

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
							return Ok(json_respond(
								BadRequest,
								json!({
									"message": "400 Bad Request",
									"error": "Query parameter \'rank\' must be 1, 2, 3, 4, 5, or *",
									"date": chrono::Utc::now().to_rfc3339(),
								}),
							));
						}
					};

					match_q
				}
				None => {
					return Ok(json_respond(
						BadRequest,
						json!({
							"message": "400 Bad Request",
							"error": "Missing query parameter: \'rank\'",
							"date": chrono::Utc::now().to_rfc3339(),
						}),
					));
				}
			};

			rank
		}

		Err(err) => {
			println!("Error: {}", err);
			return Ok(json_respond(
				UnprocessableEntity,
				json!({
					"message": "422 Unprocessable Entity",
					"error": "Malformed query parameters",
					"date": chrono::Utc::now().to_rfc3339(),
				}),
			));
		}
	};

	let repositories = tokio_run(async move {
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

	return Ok(json_respond(
		OK,
		json!({
			"message": "200 Successful",
			"date": chrono::Utc::now().to_rfc3339(),
			"count": repositories.len(),
			"data": repositories.iter().map(|repository|{
				let slug = repository.slug.clone();

				return merge_json(repository, json!({
					"refs": {
						"meta": format!("{}/jailbreak/repository/{}", env!("CANISTER_API_ENDPOINT"), slug),
						"packages": format!("{}/jailbreak/repository/{}/packages", env!("CANISTER_API_ENDPOINT"), slug),
					}
				}))
			}).collect::<Vec<Value>>(),
		}),
	));
}
