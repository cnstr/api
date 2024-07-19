use crate::{
	helpers::{clients, responses},
	types::Package,
	utility::{api_endpoint, merge_json, page_links},
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
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
	document: Package,
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
		query_by: "name,description,author,maintainer,section".to_string(),
		sort_by: "_text_match:desc".to_string(),
		page: page.to_string(),
		per_page: limit.to_string(),
	};

	let data = match clients::typesense::<TSResponse>(
		Some(query),
		"collections/packages/documents/search",
	)
	.await
	{
		Ok(data) => data,
		Err(e) => {
			eprintln!("[db] Failed to query internal search engine: {}", e);
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to query internal search engine",
			);
		}
	};

	let mut packages = data
		.hits
		.iter()
		.map(|package| {
			let package = &package.document;
			return merge_json(
				package,
				json!({
					"refs": {
						"meta": format!("{}/jailbreak/package/{}", api_endpoint(), package.package_id),
						"repo": format!("{}/jailbreak/repository/{}", api_endpoint(), package.repository_id),
					}
				}),
			);
		})
		.collect::<Vec<Value>>();

	if packages.len() > 25 {
		packages.sort_by(|a, b| {
			let a = a["quality"].as_u64().unwrap_or(0);
			let b = b["quality"].as_u64().unwrap_or(0);

			if a < 4 && b >= 4 {
				return std::cmp::Ordering::Less;
			} else if a >= 4 && b < 4 {
				return std::cmp::Ordering::Greater;
			}

			return std::cmp::Ordering::Equal;
		});
	}

	let next = packages.len() == limit as usize;
	let (prev_page, next_page) = page_links("/jailbreak/package/search", page, next);

	responses::data_with_count_and_refs(
		StatusCode::OK,
		&packages,
		packages.len(),
		json!({
			"nextPage": next_page,
			"previousPage": prev_page,
		}),
	)
}

pub async fn search_healthy() -> bool {
	let query = TSQuery {
		q: "newterm".to_string(),
		query_by: "name,description,author,maintainer,section".to_string(),
		sort_by: "_text_match:desc".to_string(),
		page: "1".to_string(),
		per_page: "100".to_string(),
	};

	match clients::typesense::<TSResponse>(Some(query), "collections/packages/documents/search")
		.await
	{
		Ok(_) => true,
		Err(_) => false,
	}
}
