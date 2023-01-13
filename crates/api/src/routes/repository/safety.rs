use once_cell::sync::OnceCell;
use serde_json::{from_str, json, Value};
use tide::{
	prelude::Deserialize,
	Request, Result,
	StatusCode::{BadRequest, Ok as OK, UnprocessableEntity},
};

use crate::utility::json_respond;

#[derive(Deserialize)]
struct Query {
	uris: Option<String>,
}

static REPOSITORIES: OnceCell<Vec<String>> = OnceCell::new();

pub async fn repository_safety(req: Request<()>) -> Result {
	if REPOSITORIES.get().is_none() {
		let raw_repositories = env!("CANISTER_PIRACY_URLS");
		let repositories = from_str::<Value>(&raw_repositories)
			.unwrap()
			.as_array()
			.unwrap()
			.iter()
			.map(|repo| repo.as_str().unwrap().to_string())
			.collect();

		REPOSITORIES.set(repositories).unwrap();
	}

	let uris = match req.query::<Query>() {
		Ok(query) => {
			let query = match query.uris {
				Some(uris) => {
					let uris: Vec<String> = uris
						.to_ascii_lowercase()
						.split(',')
						.map(|uri| uri.to_string())
						.collect();
					uris
				}
				None => {
					return Ok(json_respond(
						BadRequest,
						json!({
							"message": "400 Bad Request",
							"error": "Missing query parameter: \'uris\'",
							"date": chrono::Utc::now().to_rfc3339(),
						}),
					));
				}
			};

			query
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

	let mut repositories = Vec::new();
	let unsafe_repositories = REPOSITORIES.get().unwrap();

	for uri in uris {
		let mut is_safe = true;
		for unsafe_repository in unsafe_repositories.iter() {
			if uri.contains(unsafe_repository) {
				is_safe = false;
				break;
			}
		}

		repositories.push(json!({
			"uri": uri,
			"safe": is_safe,
		}));
	}

	return Ok(json_respond(
		OK,
		json!({
			"message": "200 Successful",
			"date": chrono::Utc::now().to_rfc3339(),
			"count": repositories.len(),
			"data": repositories,
		}),
	));
}
