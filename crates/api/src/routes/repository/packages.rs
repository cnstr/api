use crate::{
	prisma::{package, repository},
	utility::{api_respond, error_respond, handle_async, handle_prisma, merge_json, prisma},
};
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
		let repository = handle_prisma(
			prisma()
				.repository()
				.find_first(vec![
					repository::slug::equals(query.to_string()),
					repository::is_pruned::equals(false),
				])
				.exec(),
		);

		return match repository {
			Ok(repository) => match repository {
				Some(repository) => {
					match handle_prisma(
						prisma()
							.package()
							.find_many(vec![package::repository_slug::equals(repository.slug)])
							.exec(),
					) {
						Ok(packages) => Ok(packages),
						Err(err) => {
							// TODO: Sentry Error
							println!("Failed to query database: {}", err);
							return Err(error_respond(500, "Failed to query database"));
						}
					}
				}
				None => Err(error_respond(404, "Repository not found")),
			},
			Err(err) => {
				// TODO: Sentry Error
				println!("Failed to query database: {}", err);
				return Err(error_respond(500, "Failed to query database"));
			}
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
