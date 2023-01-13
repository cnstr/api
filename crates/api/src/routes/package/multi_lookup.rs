use crate::prisma::package;
use crate::utility::{merge_json, tokio_run};
use crate::{db::prisma, utility::json_respond};

use prisma_client_rust::Direction;
use serde_json::{json, Value};
use tide::{
	prelude::Deserialize,
	Request, Result,
	StatusCode::{BadRequest, NotFound, Ok as OK, UnprocessableEntity},
};

#[derive(Deserialize)]
struct Query {
	ids: Option<String>,
}

pub async fn package_multi_lookup(req: Request<()>) -> Result {
	let ids = match req.query::<Query>() {
		Ok(query) => {
			let ids = match query.ids {
				Some(ids) => {
					let ids: Vec<String> = ids.split(',').map(|id| id.to_string()).collect();
					ids
				}
				None => {
					return Ok(json_respond(
						BadRequest,
						json!({
							"message": "400 Bad Request",
							"error": "Missing query parameter: \'ids\'",
							"date": chrono::Utc::now().to_rfc3339(),
						}),
					));
				}
			};

			ids
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

	let packages = tokio_run(async move {
		return prisma()
			.package()
			.find_many(vec![
				package::package::in_vec(ids),
				package::is_current::equals(true),
				package::is_pruned::equals(false),
			])
			.order_by(package::repository_tier::order(Direction::Asc))
			.with(package::repository::fetch())
			.exec()
			.await
			.unwrap();
	});

	if packages.len() == 0 {
		return Ok(json_respond(
			NotFound,
			json!({
				"message": "404 Not Found",
				"error": "Packages not found",
				"date": chrono::Utc::now().to_rfc3339(),
			}),
		));
	}

	return Ok(json_respond(
		OK,
		json!({
			"message": "200 Successful",
			"date": chrono::Utc::now().to_rfc3339(),
			"count": packages.len(),
			"data": packages.iter().map(|package| {
				let slug = package.repository_slug.clone();
				return merge_json(package, json!({
					"refs": {
						"repo": format!("{}/jailbreak/repository/{}", env!("CANISTER_API_ENDPOINT"), slug)
					}
				}))
			}).collect::<Vec<Value>>(),
		}),
	));
}
