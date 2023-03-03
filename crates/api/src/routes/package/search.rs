use crate::{
	prisma::package,
	utility::{api_respond, error_respond, handle_typesense, merge_json, page_links},
};
use prisma_client_rust::bigdecimal::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surf::http::Method;
use tide::{Request, Result};

#[derive(Serialize, Deserialize)]
struct Query {
	q: Option<String>,
	limit: Option<u8>,
	page: Option<u8>,
}

#[derive(Deserialize, Serialize)]
struct TypesenseQuery {
	q: String,
	query_by: String,
	sort_by: String,
	page: String,
	per_page: String,
}

#[derive(Deserialize, Serialize)]
struct TypesenseResponse {
	found: u32,
	hits: Vec<Document>,
}

#[derive(Deserialize, Serialize)]
struct Document {
	document: package::Data,
}

pub async fn package_search(req: Request<()>) -> Result {
	let (query, page, limit) = match req.query::<Query>() {
		Ok(query) => {
			let q = match query.q {
				Some(q) => {
					if q.len() < 3 {
						return error_respond(
							400,
							"Query parameter \'q\' must be at least 3 characters",
						);
					}

					q
				}
				None => {
					return error_respond(400, "Missing query parameter: \'q\'");
				}
			};

			let page = match query.page {
				Some(page) => {
					if page < 1 {
						return error_respond(
							400,
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
						return error_respond(
							400,
							"Query parameter \'limit\' must be between 1 and 250",
						);
					}

					limit
				}
				None => 100,
			};

			(q, page, limit)
		}

		Err(_) => return error_respond(422, "Malformed query parameters"),
	};

	let query = TypesenseQuery {
		q: query,
		query_by: "name,description,author,maintainer,section".to_string(),
		sort_by: "_text_match:desc".to_string(),
		page: page.to_string(),
		per_page: limit.to_string(),
	};

	let response = match handle_typesense::<TypesenseQuery, TypesenseResponse>(
		query,
		"/collections/packages/documents/search",
		Method::Get,
	)
	.await
	{
		Ok(data) => data,
		Err(err) => return err,
	};

	let mut packages = response
			.hits
            .iter()
			.map(|package| {
				let package = &package.document;
				return merge_json(
					package,
					json!({
						"refs": {
							"meta": format!("{}/jailbreak/package/{}", env!("CANISTER_API_ENDPOINT"), package.package),
							"repo": format!("{}/jailbreak/repository/{}", env!("CANISTER_API_ENDPOINT"), package.repository_slug),
						}
					}),
				);
			})
			.collect::<Vec<Value>>();

	if packages.len() > 25 {
		packages.sort_by(|a, b| {
			let a = a["repositoryTier"].as_u64().unwrap_or(0);
			let b = b["repositoryTier"].as_u64().unwrap_or(0);

			if a < 4 && b >= 4 {
				return std::cmp::Ordering::Less;
			} else if a >= 4 && b < 4 {
				return std::cmp::Ordering::Greater;
			}

			return std::cmp::Ordering::Equal;
		});
	}

	let next = packages.len().to_u8().unwrap_or(0) == limit;
	let (prev_page, next_page) = page_links("/jailbreak/package/search", page, next);

	api_respond(
		200,
		json!({
			"refs": {
				"nextPage": next_page,
				"previousPage": prev_page,
			},
			"count": packages.len(),
			"data": packages,
		}),
	)
}

pub async fn package_search_healthy() -> bool {
	match handle_typesense::<TypesenseQuery, TypesenseResponse>(
		TypesenseQuery {
			q: "newterm".to_string(),
			query_by: "name,description,author,maintainer,section".to_string(),
			sort_by: "_text_match:desc".to_string(),
			page: "1".to_string(),
			per_page: "100".to_string(),
		},
		"/collections/packages/documents/search",
		Method::Get,
	)
	.await
	{
		Ok(_) => true,
		Err(_) => false,
	}
}
