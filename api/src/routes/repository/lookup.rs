use crate::prisma::repository;
use crate::utility::json_respond;
use crate::{db::prisma, utility::merge_json};

use serde_json::json;
use tide::{
	Request, Result,
	StatusCode::{BadRequest, NotFound, Ok as OK},
};
use tokio::runtime::Builder;

pub async fn repository_lookup(req: Request<()>) -> Result {
	let query = match req.param("repository") {
		Ok(query) => query.to_string(),
		Err(_) => {
			return Ok(json_respond(
				BadRequest,
				json!({
					"message": "400 Bad Request",
					"error": "Missing URL parameter: \':repository\'",
					"date": chrono::Utc::now().to_rfc3339(),
				}),
			));
		}
	};

	let repository = Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async move {
			return prisma()
				.await
				.repository()
				.find_first(vec![
					repository::slug::equals(query.to_string()),
					repository::is_pruned::equals(false),
				])
				.with(repository::origin::fetch())
				.exec()
				.await
				.unwrap();
		});

	match repository {
		Some(repository) => {
			let slug = repository.slug.clone();
			let repository = merge_json(
				repository,
				json!({
					"refs": {
						"packages": format!("{}/jailbreak/repository/{}/packages", env!("CANISTER_API_ENDPOINT"), slug),
					}
				}),
			);

			return Ok(json_respond(
				OK,
				json!({
					"message": "200 Successful",
					"date": chrono::Utc::now().to_rfc3339(),
					"data": repository,
				}),
			));
		}
		None => {
			return Ok(json_respond(
				NotFound,
				json!({
					"message": "404 Not Found",
					"error": "Repository not found",
					"date": chrono::Utc::now().to_rfc3339(),
				}),
			))
		}
	}
}
