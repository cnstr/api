use once_cell::sync::OnceCell;
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client, Error as ReqwestError,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Display, process::exit, time::Duration};

static TYPESENSE_CLIENT: OnceCell<Client> = OnceCell::new();

pub struct TypesenseQueryError {
	pub message: String,
	pub http_error: Option<ReqwestError>,
}

impl Display for TypesenseQueryError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl TypesenseQueryError {
	fn from_str(message: &str) -> Self {
		Self {
			message: message.to_string(),
			http_error: None,
		}
	}

	fn from_reqwest(message: &str, http_error: ReqwestError) -> Self {
		Self {
			message: message.to_string(),
			http_error: Some(http_error),
		}
	}
}

fn typesense_client() -> &'static Client {
	match TYPESENSE_CLIENT.get_or_try_init(|| {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Typesense-API-Key",
			HeaderValue::from_static(env!("CANISTER_TYPESENSE_KEY")),
		);

		Client::builder().timeout(Duration::from_secs(10)).build()
	}) {
		Ok(client) => client,
		Err(err) => {
			// TODO: Report Error
			println!("panic: failed to create typesense client: {}", err);
			exit(1);
		}
	}
}

pub async fn typesense<R: DeserializeOwned>(
	query: impl Serialize,
	path: &str,
) -> Result<R, TypesenseQueryError> {
	let url = format!("http://{}:8108/{}", env!("CANISTER_TYPESENSE_HOST"), path);
	let request = typesense_client().get(&url).query(&query);

	let response = match request.send().await {
		Ok(responses) => responses,
		Err(err) => {
			// TODO: Report Error
			return Err(TypesenseQueryError::from_reqwest(
				"failed to send request",
				err,
			));
		}
	};

	if !response.status().is_success() {
		// TODO: Report Error
		return Err(TypesenseQueryError::from_reqwest(
			"failed to send request",
			ReqwestError::from(response.error_for_status().unwrap_err()),
		));
	}

	let response = match response.json::<R>().await {
		Ok(response) => response,
		Err(err) => {
			// TODO: Report Error
			return Err(TypesenseQueryError::from_str("failed to parse response"));
		}
	};

	Ok(response)
}
