use crate::{
	helpers::{clients, responses},
	prisma::repository,
	utility::{api_endpoint, merge_json, page_links},
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use prisma_client_rust::bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
pub struct SearchParams {
	q: Option<String>,
	limit: Option<u8>,
	page: Option<u8>,
}

#[derive(Deserialize, Serialize)]
struct TSQuery {
	q: String,
	query_by: String,
	sort_by: String,
	page: String,
	per_page: String,
}

#[derive(Deserialize, Serialize)]
struct TSResponse {
	found: u32,
	hits: Vec<Document>,
}

#[derive(Deserialize, Serialize)]
struct Document {
	document: repository::Data,
}

pub async fn search(query: Query<SearchParams>) -> impl IntoResponse {
	let q = match &query.q {
		Some(q) => {
			if q.len() < 2 {
				return responses::error(
					StatusCode::BAD_REQUEST,
					"Query parameter \'q\' must be at least 2 characters",
				);
			}

			q
		}

		None => {
			return responses::error(StatusCode::BAD_REQUEST, "Missing query parameter: \'q\'");
		}
	};

	let page = match query.page {
		Some(page) => {
			if page < 1 {
				return responses::error(
					StatusCode::BAD_REQUEST,
					"Query parameter \'page\' must be greater than 0",
				);
			}

			page
		}

		None => 1,
	};

	let limit = match query.limit {
		Some(limit) => {
			if !(1..=250).contains(&limit) {
				return responses::error(
					StatusCode::BAD_REQUEST,
					"Query parameter \'limit\' must be between 1 and 250",
				);
			}

			limit
		}

		None => 100,
	};

	let query = TSQuery {
		q: q.to_string(),
		query_by: "slug,name,description,aliases".to_string(),
		sort_by: "tier:asc".to_string(),
		page: page.to_string(),
		per_page: limit.to_string(),
	};

	let data = match clients::typesense::<TSResponse>(
		Some(query),
		"collections/repositories/documents/search",
	)
	.await
	{
		Ok(data) => data,
		Err(_) => {
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to query internal search engine",
			);
		}
	};

	let repositories = data
		.hits
		.iter()
		.map(|repository| {
			let repository = &repository.document;
			return merge_json(
				repository,
				json!({
					"refs": {
						"meta": format!("{}/jailbreak/repository/{}", api_endpoint(), repository.slug),
						"packages": format!("{}/jailbreak/repository/{}/packages", api_endpoint(), repository.slug),
					}
				}),
			);
		})
		.collect::<Vec<Value>>();

	let next = repositories.len().to_u8().unwrap_or(0) == limit;
	let (prev_page, next_page) = page_links("/jailbreak/repository/search", page, next);

	responses::data_with_count_and_refs(
		StatusCode::OK,
		&repositories,
		repositories.len(),
		json!({
			"nextPage": next_page,
			"previousPage": prev_page,
		}),
	)
}

pub async fn search_healthy() -> bool {
	let query = TSQuery {
		q: "chariz".to_string(),
		query_by: "slug,name,description,aliases".to_string(),
		sort_by: "tier:asc".to_string(),
		page: "1".to_string(),
		per_page: "100".to_string(),
	};

	match clients::typesense::<TSResponse>(Some(query), "collections/repositories/documents/search")
		.await
	{
		Ok(_) => true,
		Err(_) => false,
	}
}
