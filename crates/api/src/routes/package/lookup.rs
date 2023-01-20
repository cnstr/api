use crate::db::prisma;
use crate::prisma::package;
use crate::utility::{api_respond, error_respond, handle_async, merge_json};
use prisma_client_rust::Direction;
use serde_json::{json, Value};
use tide::{Request, Result};

pub async fn package_lookup(req: Request<()>) -> Result {
	let query = match req.param("package") {
		Ok(query) => query.to_string(),
		Err(_) => {
			return error_respond(400, "Missing URL parameter: \':package\'");
		}
	};

	let packages = handle_async(async move {
		return prisma()
			.package()
			.find_many(vec![
				package::package::equals(query.to_string()),
				package::is_pruned::equals(false),
			])
			.order_by(package::is_current::order(Direction::Desc))
			.order_by(package::repository_tier::order(Direction::Asc))
			.with(package::repository::fetch())
			.exec()
			.await
			.unwrap();
	});

	if packages.len() == 0 {
		return error_respond(404, "Package not found");
	}

	api_respond(
		200,
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
	)
}
