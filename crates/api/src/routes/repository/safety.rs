use crate::{
	helpers::responses,
	utility::{handle_error, load_runtime_config},
};
use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

#[derive(Deserialize)]
pub struct SafetyParams {
	uris: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct Repositories {
	repositories: Vec<String>,
}

// TODO: Move into method body and implement new system like in goblar
static REPOSITORIES: OnceCell<Vec<String>> = OnceCell::new();

pub async fn safety(query: Query<SafetyParams>) -> impl IntoResponse {
	if !set_repositories().await {
		return responses::error(
			StatusCode::INTERNAL_SERVER_ERROR,
			"Unable to fetch repository list",
		);
	}

	let uris = match &query.uris {
		Some(uris) => {
			let uris = uris
				.to_ascii_lowercase()
				.split(',')
				.map(|uri| uri.to_string())
				.collect::<Vec<String>>();
			uris
		}

		None => {
			return responses::error(StatusCode::BAD_REQUEST, "Missing query parameter: \'uris\'")
		}
	};

	let mut repositories = Vec::new();
	let unsafe_repositories = match REPOSITORIES.get() {
		Some(repositories) => repositories,
		None => {
			// TODO: Report Error
			println!("Failed to get repository list (REPOSITORIES.get() returned None)");
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to get repository list",
			);
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

	responses::data_with_count(StatusCode::OK, &repositories, repositories.len())
}

async fn set_repositories() -> bool {
	if REPOSITORIES.get().is_none() {
		let config = load_runtime_config();
		let response = match reqwest::get(config.piracy_url).await {
			Ok(response) => response,
			Err(e) => panic!("Failed to fetch piracy URLs ({e})"),
		};

		let value = match response.text().await {
			Ok(value) => value,
			Err(e) => panic!("Failed to parse piracy URLs ({e})"),
		};

		let repositories = match from_str(&value) {
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

pub async fn safety_healthy() -> bool {
	let result = set_repositories().await;
	if result == false {
		return false;
	}

	let test_safe = "https://havoc.app";
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
