use crate::{
	db::{elastic, prisma},
	utility::json_respond,
};

use elasticsearch::SearchParts;
use prisma::repository;
use serde_json::json;
use tide::{
	prelude::Deserialize,
	Request, Result,
	StatusCode::{BadRequest, InternalServerError, Ok as OK, UnprocessableEntity},
};
use tokio::runtime::Builder;

#[derive(Deserialize)]
struct Query {
	q: Option<String>,
	limit: Option<u8>,
	page: Option<u8>,
}

pub async fn package_search(req: Request<()>) -> Result {
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

	let request = Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async move {
			let elastic_request = elastic()
				.await
				.search(SearchParts::Index(&["packages"]))
				.body(json!({
					"query": {
						"query_string": {
							"fields": ["name", "description", "author", "maintainer", "section"],
							"fuzziness": "AUTO",
							"fuzzy_transpositions": true,
							"fuzzy_max_expansions": 100,
							"query": format!("{}~", query),
						}
					},
					"sort": {
						"repositoryTier": {
							"order": "asc"
						},
						"_score": {
							"order": "desc"
						}
					},
					"from": (page - 1) * limit,
					"size": limit
				}))
				.send()
				.await;

			let elastic_response = match elastic_request {
				Ok(res) => res,
				Err(err) => {
					println!("Error: {}", err);
					return Err(Ok(json_respond(
						InternalServerError,
						json!({
							"message": "500 Internal Server Error",
							"error": "Failed to connect to Elasticsearch",
							"date": chrono::Utc::now().to_rfc3339(),
						}),
					)));
				}
			};

			let json = elastic_response.json::<serde_json::Value>().await.unwrap();
			let packages = json["hits"]["hits"].as_array().unwrap();

			let repository_slugs = packages
				.iter()
				.map(|package| {
					package["_source"]["repositorySlug"]
						.as_str()
						.unwrap()
						.to_string()
				})
				.collect::<Vec<String>>();

			let repositories = prisma()
				.await
				.repository()
				.find_many(vec![repository::slug::in_vec(repository_slugs)])
				.exec()
				.await
				.unwrap();

			let packages = packages
				.iter()
				.map(|package| {
					let repository = repositories
						.iter()
						.find(|repository| {
							repository.slug
								== package["_source"]["repositorySlug"].as_str().unwrap()
						})
						.unwrap();

					let mut package = package["_source"].clone();
					package["repository"] = json!(repository);
					package
				})
				.collect::<Vec<serde_json::Value>>();

			Ok(packages)
		});

	let packages = match request {
		Ok(packages) => packages,
		Err(err) => return err,
	};

	return Ok(json_respond(
		OK,
		json!({
			"message": "200 Successful",
			"date": chrono::Utc::now().to_rfc3339(),
			"count": packages.len(),
			"data": packages,
		}),
	));
}
