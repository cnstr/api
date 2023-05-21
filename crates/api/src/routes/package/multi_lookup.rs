use crate::{
	helpers::{clients, responses},
	prisma::package,
	utility::merge_json,
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use prisma_client_rust::Direction;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct MultiLookupParams {
	ids: Option<String>,
}

pub async fn multi_lookup(query: Query<MultiLookupParams>) -> impl IntoResponse {
	let ids = match &query.ids {
		Some(ids) => {
			let ids: Vec<String> = ids.split(',').map(|id| id.to_string()).collect();
			ids
		}
		None => {
			return responses::error(StatusCode::BAD_REQUEST, "Missing query parameter: \'ids\'");
		}
	};

	let packages = match clients::prisma(|prisma| {
		prisma
			.package()
			.find_many(vec![
				package::package::in_vec(ids),
				package::is_current::equals(true),
				package::is_pruned::equals(false),
			])
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
		return responses::error(StatusCode::NOT_FOUND, "Packages not found");
	}

	let mut ids: Vec<String> = packages
		.iter()
		.map(|package| package.package.clone())
		.collect();

	responses::data_with_count(
		StatusCode::OK,
		packages
			.iter()
			.filter(|package| {
				let package = package.package.clone();
				if ids.contains(&package) {
					ids.retain(|id| id != &package);
					return true;
				}
				return false;
			})
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

pub async fn multi_lookup_healthy() -> bool {
	match clients::prisma(|prisma| {
		prisma
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
			.exec()
	})
	.await
	{
		Ok(_) => true,
		Err(_) => false,
	}
}
