use serde_json::{json, to_string_pretty};
use tide::{
	prelude::Deserialize,
	Request, Response, Result,
	StatusCode::{BadRequest, UnprocessableEntity},
};

#[derive(Deserialize)]
struct Query {
	uris: Option<String>,
}

pub async fn repository_safety(req: Request<()>) -> Result {
	let unsafe_repositories = vec!["repo.hackyouriphone.org"];

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
					return Ok(Response::builder(BadRequest)
						.body(
							to_string_pretty(&json!({
								"message": "400 Bad Request",
								"error": "Missing query parameter: \'uris\'",
								"date": chrono::Utc::now().to_rfc3339(),
							}))
							.unwrap(),
						)
						.build());
				}
			};

			query
		}

		Err(err) => {
			println!("Error: {}", err);
			return Ok(Response::builder(UnprocessableEntity)
				.body(
					to_string_pretty(&json!({
						"message": "422 Unprocessable Entity",
						"error": "Malformed query parameters",
						"date": chrono::Utc::now().to_rfc3339(),
					}))
					.unwrap(),
				)
				.build());
		}
	};

	let mut repositories = Vec::new();

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

	return Ok(to_string_pretty(&json!({
		"message": "200 Successful",
		"date": chrono::Utc::now().to_rfc3339(),
		"count": repositories.len(),
		"data": repositories,
	}))
	.unwrap()
	.into());
}
