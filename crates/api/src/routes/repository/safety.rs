use crate::utility::{api_respond, error_respond, handle_error};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use tide::{Request, Result};

#[derive(Serialize, Deserialize)]
struct Query {
	uris: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct Repositories {
	repositories: Vec<String>,
}

static REPOSITORIES: OnceCell<Vec<String>> = OnceCell::new();

pub async fn repository_safety(req: Request<()>) -> Result {
	match set_repositories() {
		true => {}
		false => {
			return error_respond(500, "Unable to fetch repository list");
		}
	}

	let uris = match req.query::<Query>() {
		Ok(query) => {
			let query = match query.uris {
				Some(uris) => {
					let uris = uris
						.to_ascii_lowercase()
						.split(',')
						.map(|uri| uri.to_string())
						.collect::<Vec<String>>();
					uris
				}
				None => return error_respond(400, "Missing query parameter: \'uris\'"),
			};

			query
		}

		Err(err) => {
			println!("Error: {}", err);
			return error_respond(422, "Malformed query parameters");
		}
	};

	let mut repositories = Vec::new();
	let unsafe_repositories = match REPOSITORIES.get() {
		Some(repositories) => repositories,
		None => {
			println!("Failed to get repository list (REPOSITORIES.get() returned None)");
			return error_respond(500, "Unable to fetch repository list");
		}
	};

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

	api_respond(
		200,
		json!({
			"count": repositories.len(),
			"data": repositories,
		}),
	)
}

fn set_repositories() -> bool {
	if REPOSITORIES.get().is_none() {
		let raw_repositories = env!("CANISTER_PIRACY_URLS");
		let repositories = match from_str(raw_repositories) {
			Ok(repositories) => {
				let data: Repositories = repositories;
				data.repositories
			}
			Err(err) => {
				handle_error(&err.into());
				return false;
			}
		};

		match REPOSITORIES.set(repositories) {
			Ok(_) => {}
			Err(_) => println!("Repository list already set"),
		};
	}

	true
}

pub async fn repository_safety_healthy() -> bool {
    let result = set_repositories();
    if result == false {
        return false;
    }

	let test_safe = "https://repo.chariz.com";
	let test_unsafe = "https://repo.hackyouriphone.org";

	let repositories = match REPOSITORIES.get() {
		Some(repositories) => repositories,
		None => {
			println!("Failed to get repository list (REPOSITORIES.get() returned None)");
			return false;
		}
	};

	let mut safe_pass = true;
	let mut unsafe_pass = false;

	for unsafe_repository in repositories.iter() {
		if test_safe.contains(unsafe_repository) {
			safe_pass = false;
		}

		if test_unsafe.contains(unsafe_repository) {
			unsafe_pass = true;
		}
	}

	safe_pass && unsafe_pass
}
