use crate::{
	prisma::package,
	utility::{api_respond, error_respond, handle_async, merge_json, page_links, typesense},
};
use prisma_client_rust::bigdecimal::ToPrimitive;
use serde::Serialize;
use serde_json::{json, Value};
use tide::{prelude::Deserialize, Request, Result};

#[derive(Deserialize)]
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
					if limit < 1 || limit > 250 {
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

		Err(err) => {
			println!("Error: {}", err);
			return error_respond(422, "Malformed query parameters");
		}
	};

	let request = handle_async(async move {
		let query = TypesenseQuery {
			q: query,
			query_by: "name,description,author,maintainer,section".to_string(),
			sort_by: "repositoryTier:asc".to_string(),
			page: page.to_string(),
			per_page: limit.to_string(),
		};

		let request = typesense()
			.get("/collections/packages/documents/search")
			.query(&query);

		let request = match request {
			Ok(request) => request,
			Err(err) => {
				println!("Error: {}", err);
				return Err(error_respond(500, "Failed to build Typesense query"));
			}
		};

		let response = match typesense().recv_json(request).await {
			Ok(response) => {
				let response: TypesenseResponse = response;
				response
			}
			Err(err) => {
				println!("Error: {}", err);
				return Err(error_respond(500, "Failed to send Typesense query"));
			}
		};

		let packages = response
			.hits
			.iter()
			.map(|package| {
				let package = &package.document;
				return merge_json(
					&package,
					json!({
						"refs": {
							"meta": format!("{}/jailbreak/package/{}", env!("CANISTER_API_ENDPOINT"), package.package),
							"repo": format!("{}/jailbreak/repository/{}", env!("CANISTER_API_ENDPOINT"), package.repository_slug),
						}
					}),
				);
			})
			.collect::<Vec<Value>>();

		return Ok(packages);
	});

	let packages = match request {
		Ok(packages) => packages,
		Err(response) => return response,
	};

	let next = packages.len().to_u8().unwrap() == limit;
	let (prev_page, next_page) = page_links("/jailbreak/package/search", page, next);

	return api_respond(
		200,
		json!({
			"refs": {
				"nextPage": next_page,
				"previousPage": prev_page,
			},
			"count": packages.len(),
			"data": packages,
		}),
	);
}
