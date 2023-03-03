use crate::{
	prisma::package,
	utility::{api_respond, error_respond, handle_prisma, merge_json, prisma},
};
use prisma_client_rust::Direction;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tide::{Request, Result};

#[derive(Serialize, Deserialize)]
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

		Err(_) => return error_respond(422, "Malformed query parameters"),
	};

	let packages = match handle_prisma(
		prisma()
			.package()
			.find_many(vec![
				package::package::in_vec(ids),
				package::is_current::equals(true),
				package::is_pruned::equals(false),
			])
			.order_by(package::repository_tier::order(Direction::Asc))
			.with(package::repository::fetch())
			.exec(),
	) {
		Ok(packages) => packages,
		Err(err) => return err,
	};

	if packages.is_empty() {
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

pub async fn package_multi_lookup_healthy() -> bool {
	match handle_prisma(
		prisma()
			.package()
			.find_many(vec![
				package::package::in_vec(vec![
					"ws.hbang.common".to_string(),
					"me.renai.lyricify".to_string(),
				]),
				package::is_current::equals(true),
				package::is_pruned::equals(false),
			])
			.order_by(package::repository_tier::order(Direction::Asc))
			.with(package::repository::fetch())
			.exec(),
	) {
		Ok(_) => true,
		Err(_) => false,
	}
}
