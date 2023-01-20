use crate::prisma::repository;
use crate::utility::{api_respond, error_respond, tokio_run};
use crate::{db::prisma, utility::merge_json};
use serde_json::json;
use tide::{Request, Result};

pub async fn repository_lookup(req: Request<()>) -> Result {
	let query = match req.param("repository") {
		Ok(query) => query.to_string(),
		Err(_) => {
			return error_respond(400, "Missing URL parameter: \':repository\'");
		}
	};

	let repository = tokio_run(async move {
		return prisma()
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

			return api_respond(
				200,
				json!({
					"data": repository,
				}),
			);
		}
		None => {
			return error_respond(404, "Repository not found");
		}
	}
}
