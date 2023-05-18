use crate::{
	helpers::{clients, responses},
	prisma::package,
	utility::merge_json,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use prisma_client_rust::Direction;
use serde_json::{json, Value};

pub async fn lookup(package: Path<String>) -> impl IntoResponse {
	let packages = match clients::prisma(|prisma| {
		prisma
			.package()
			.find_many(vec![
				package::package::equals(package.to_string()),
				package::is_pruned::equals(false),
			])
			.order_by(package::is_current::order(Direction::Desc))
			.order_by(package::repository_tier::order(Direction::Asc))
			.with(package::repository::fetch())
			.exec()
	})
	.await
	{
		Ok(packages) => packages,
		Err(_) => {
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to query database",
			);
		}
	};

	if packages.is_empty() {
		return responses::error(StatusCode::NOT_FOUND, "Package not found");
	}

	responses::data_with_count(
		StatusCode::OK,
		packages
			.iter()
			.map(|package| {
				let slug = package.repository_slug.clone();
				return merge_json(
					package,
					json!({
						"refs": {
							"repo": format!("{}/jailbreak/repository/{}", env!("CANISTER_API_ENDPOINT"), slug)
						}
					}),
				);
			})
			.collect::<Vec<Value>>(),
		packages.len(),
	)
}

pub async fn lookup_healthy() -> bool {
	match clients::prisma(|prisma| {
		prisma
			.package()
			.find_many(vec![
				package::package::equals("ws.hbang.common".to_string()),
				package::is_pruned::equals(false),
			])
			.order_by(package::is_current::order(Direction::Desc))
			.order_by(package::repository_tier::order(Direction::Asc))
			.with(package::repository::fetch())
			.exec()
	})
	.await
	{
		Ok(_) => true,
		Err(_) => false,
	}
}
