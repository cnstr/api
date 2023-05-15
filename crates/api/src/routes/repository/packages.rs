use crate::{
	helpers::responses,
	prisma::{package, repository},
	utility::{handle_prisma, merge_json, prisma},
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::{json, Value};

pub async fn packages(repository: Path<String>) -> impl IntoResponse {
	let repository = prisma()
		.repository()
		.find_first(vec![
			repository::slug::equals(repository.to_string()),
			repository::is_pruned::equals(false),
		])
		.exec()
		.await;

	let packages = match repository {
		Ok(repository) => match repository {
			Some(repository) => {
				match handle_prisma(
					prisma()
						.package()
						.find_many(vec![package::repository_slug::equals(repository.slug)])
						.exec(),
				) {
					Ok(packages) => packages
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
						.collect::<Vec<Value>>(),

					Err(err) => {
						// TODO: Report Error
						return responses::error(
							StatusCode::INTERNAL_SERVER_ERROR,
							"Failed to query database",
						);
					}
				}
			}

			None => return responses::error(StatusCode::NOT_FOUND, "Repository not found"),
		},
		Err(err) => {
			// TODO: Report Error
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to query database",
			);
		}
	};

	responses::data_with_count(StatusCode::OK, &packages, packages.len())
}

pub async fn packages_healthy() -> bool {
	match prisma()
		.repository()
		.find_first(vec![
			repository::slug::equals("chariz".to_string()),
			repository::is_pruned::equals(false),
		])
		.exec()
		.await
	{
		Ok(repository) => match repository {
			Some(repository) => {
				match prisma()
					.package()
					.find_many(vec![package::repository_slug::equals(repository.slug)])
					.exec()
					.await
				{
					Ok(_) => true,
					Err(_) => false,
				}
			}

			None => false,
		},

		Err(_) => false,
	}
}
