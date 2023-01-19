use crate::{
	db::typesense,
	prisma::repository,
	utility::{json_respond, merge_json, page_links, tokio_run},
};

use prisma_client_rust::bigdecimal::ToPrimitive;
use serde::Serialize;
use serde_json::{json, Value};
use tide::{
	prelude::Deserialize,
	Request, Result,
	StatusCode::{BadRequest, InternalServerError, Ok as OK, UnprocessableEntity},
};

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
	document: repository::Data,
}

pub async fn repository_search(req: Request<()>) -> Result {
	let (query, page, limit) = match req.query::<Query>() {
		Ok(query) => {
			let q = match query.q {
				Some(q) => {
					if q.len() < 3 {
						return Ok(json_respond(
							BadRequest,
							json!({
								"message": "400 Bad Request",
								"error": "Query parameter \'q\' must be at least 3 characters",
								"date": chrono::Utc::now().to_rfc3339(),
							}),
						));
					}

					q
				}
				None => {
					return Ok(json_respond(
						BadRequest,
						json!({
							"message": "400 Bad Request",
							"error": "Missing query parameter: \'q\'",
							"date": chrono::Utc::now().to_rfc3339(),
						}),
					));
				}
			};

			let page = match query.page {
				Some(page) => {
					if page < 1 {
						return Ok(json_respond(
							BadRequest,
							json!({
								"message": "400 Bad Request",
								"error": "Query parameter \'page\' must be greater than 0",
								"date": chrono::Utc::now().to_rfc3339(),
							}),
						));
					}

					page
				}
				None => 1,
			};

			let limit = match query.limit {
				Some(limit) => {
					if limit < 1 || limit > 250 {
						return Ok(json_respond(
							BadRequest,
							json!({
								"message": "400 Bad Request",
								"error": "Query parameter \'limit\' must be between 1 and 250",
								"date": chrono::Utc::now().to_rfc3339(),
							}),
						));
					}

					limit
				}
				None => 100,
			};

			(q, page, limit)
		}

		Err(err) => {
			println!("Error: {}", err);
			return Ok(json_respond(
				UnprocessableEntity,
				json!({
					"message": "422 Unprocessable Entity",
					"error": "Malformed query parameters",
					"date": chrono::Utc::now().to_rfc3339(),
				}),
			));
		}
	};

	let request = tokio_run(async move {
		let query = TypesenseQuery {
			q: query,
			query_by: "slug,name,description,aliases".to_string(),
			sort_by: "tier:asc".to_string(),
			page: page.to_string(),
			per_page: limit.to_string(),
		};

		let request = typesense()
			.get("/collections/repositories/documents/search")
			.query(&query);

		let request = match request {
			Ok(request) => request,
			Err(err) => {
				println!("Error: {}", err);
				return Err(json_respond(
					InternalServerError,
					json!({
						"message": "500 Internal Server Error",
						"error": "Failed to build Typesense query",
						"date": chrono::Utc::now().to_rfc3339(),
					}),
				));
			}
		};

		let response = match typesense().recv_json(request).await {
			Ok(response) => {
				let response: TypesenseResponse = response;
				response
			}
			Err(err) => {
				println!("Error: {}", err);
				return Err(json_respond(
					InternalServerError,
					json!({
						"message": "500 Internal Server Error",
						"error": "Failed to send Typesense query",
						"date": chrono::Utc::now().to_rfc3339(),
					}),
				));
			}
		};

		let repositories = response
			.hits
			.iter()
			.map(|repository| {
				let repository = &repository.document;
				return merge_json(
					&repository,
					json!({
						"refs": {
							"meta": format!("{}/jailbreak/repository/{}", env!("CANISTER_API_ENDPOINT"), repository.slug),
							"packages": format!("{}/jailbreak/repository/{}/packages", env!("CANISTER_API_ENDPOINT"), repository.slug),
						}
					}),
				);
			})
			.collect::<Vec<Value>>();

		Ok(repositories)
	});

	let repositories = match request {
		Ok(repositories) => repositories,
		Err(err) => return Ok(err),
	};

	let url = req.url().path();
	let next = repositories.len().to_u8().unwrap() == limit;
	let (prev_page, next_page) = page_links(url, page, next);

	return Ok(json_respond(
		OK,
		json!({
			"message": "200 OK",
			"date": chrono::Utc::now().to_rfc3339(),
			"refs": {
				"nextPage": next_page,
				"previousPage": prev_page,
			},
			"count": repositories.len(),
			"data": repositories
		}),
	));
}
