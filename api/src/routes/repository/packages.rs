use crate::prisma::repository;
use crate::utility::{json_respond, merge_json, tokio_run};
use crate::{db::prisma, prisma::package};

use serde_json::{json, Value};
use tide::{
	Request, Result,
	StatusCode::{BadRequest, NotFound, Ok as OK},
};

pub async fn repository_packages(req: Request<()>) -> Result {
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

	let request = tokio_run(async move {
		let repository = prisma()
			.await
			.repository()
			.find_first(vec![
				repository::slug::equals(query.to_string()),
				repository::is_pruned::equals(false),
			])
			.exec()
			.await
			.unwrap();

		return match repository {
			Some(repository) => Ok(prisma()
				.await
				.package()
				.find_many(vec![package::repository_slug::equals(repository.slug)])
				.exec()
				.await
				.unwrap()),
			None => Err(Ok(json_respond(
				NotFound,
				json!({
					"message": "404 Not Found",
					"error": "Repository not found",
					"date": chrono::Utc::now().to_rfc3339(),
				}),
			))),
		};
	});

	let packages = match request {
		Ok(packages) => {
			let packages = packages
				.into_iter()
				.map(|package| {
					let id = package.package.clone();
					let slug = package.repository_slug.clone();

					merge_json(
						package,
						json!({
							"refs": {
								"meta": format!("{}/jailbreak/package/{}", env!("CANISTER_API_ENDPOINT"), id),
								"repo": format!("{}/jailbreak/repository/{}", env!("CANISTER_API_ENDPOINT"), slug),
							}
						}),
					)
				})
				.collect::<Vec<Value>>();

			packages
		}
		Err(response) => return response,
	};

	return Ok(json_respond(
		OK,
		json!({
			"message": "200 Successful",
			"date": chrono::Utc::now().to_rfc3339(),
			"count": packages.len(),
			"data": packages,
		}),
	));
}
