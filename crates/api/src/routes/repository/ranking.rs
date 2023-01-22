use crate::{
	prisma::repository,
	utility::{api_respond, error_respond, handle_async, handle_prisma, merge_json, prisma},
};
use prisma_client_rust::Direction;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tide::{Request, Result};

#[derive(Serialize, Deserialize)]
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
							return error_respond(
								400,
								"Query parameter \'rank\' must be 1, 2, 3, 4, 5, or *",
							)
						}
					};

					match_q
				}
				None => {
					return error_respond(400, "Missing query parameter: \'rank\'");
				}
			};

			rank
		}

		Err(err) => {
			println!("Error: {}", err);
			return error_respond(422, "Malformed query parameters");
		}
	};

	let repositories = handle_async(async move {
		return match query.as_str() {
			"*" => handle_prisma(
				prisma()
					.repository()
					.find_many(vec![repository::is_pruned::equals(false)])
					.order_by(repository::tier::order(Direction::Asc))
					.with(repository::origin::fetch())
					.exec(),
			),

			_ => handle_prisma(
				prisma()
					.repository()
					.find_many(vec![
						repository::tier::equals(query.parse::<i32>().unwrap_or_else(|err| {
							println!("Error: {}", err);
							return 1;
						})),
						repository::is_pruned::equals(false),
					])
					.order_by(repository::tier::order(Direction::Asc))
					.with(repository::origin::fetch())
					.exec(),
			),
		};
	});

	let repositories = match repositories {
		Ok(repositories) => repositories,
		Err(err) => {
			println!("Error: {}", err);
			return error_respond(500, "Internal server error");
		}
	};

	return api_respond(
		200,
		json!({
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
	);
}
