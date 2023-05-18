use crate::{
	helpers::{clients, responses},
	prisma::repository,
	utility::merge_json,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn lookup(respository: Path<String>) -> impl IntoResponse {
	let repository = match clients::prisma(|prisma| {
		prisma
			.repository()
			.find_first(vec![
				repository::slug::equals(respository.to_string()),
				repository::is_pruned::equals(false),
			])
			.with(repository::origin::fetch())
			.exec()
	})
	.await
	{
		Ok(repository) => repository,
		Err(_) => {
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to query database",
			);
		}
	};

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

			responses::data(StatusCode::OK, repository)
		}

		None => responses::error(StatusCode::NOT_FOUND, "Repository not found"),
	}
}

pub async fn lookup_healthy() -> bool {
	match clients::prisma(|prisma| {
		prisma
			.repository()
			.find_first(vec![
				repository::slug::equals("chariz".to_string()),
				repository::is_pruned::equals(false),
			])
			.exec()
	})
	.await
	{
		Ok(_) => true,
		Err(_) => false,
	}
}
