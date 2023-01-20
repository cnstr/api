use crate::prisma::repository;
use crate::utility::{api_respond, error_respond, handle_async, merge_json};
use crate::{db::prisma, prisma::package};
use serde_json::{json, Value};
use tide::{Request, Result};

pub async fn repository_packages(req: Request<()>) -> Result {
	let query = match req.param("repository") {
		Ok(query) => query.to_string(),
		Err(_) => {
			return error_respond(400, "Missing URL parameter: \':repository\'");
		}
	};

	let request = handle_async(async move {
		let repository = prisma()
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
				.package()
				.find_many(vec![package::repository_slug::equals(repository.slug)])
				.exec()
				.await
				.unwrap()),
			None => Err(error_respond(404, "Repository not found")),
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

	return api_respond(
		200,
		json!({
			"count": packages.len(),
				"data": packages,
		}),
	);
}
