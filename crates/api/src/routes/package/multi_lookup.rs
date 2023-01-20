use crate::db::prisma;
use crate::prisma::package;
use crate::utility::{api_respond, error_respond, handle_async, merge_json};
use prisma_client_rust::Direction;
use serde_json::{json, Value};
use tide::{prelude::Deserialize, Request, Result};

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
					return error_respond(400, "Missing query parameter: \'ids\'");
				}
			};

			ids
		}

		Err(err) => {
			println!("Error: {}", err);
			return error_respond(422, "Malformed query parameters");
		}
	};

	let packages = handle_async(async move {
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
		return error_respond(400, "Packages not found");
	}

	return api_respond(
		200,
		json!({
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
	);
}
