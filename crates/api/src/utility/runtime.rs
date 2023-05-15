use super::typesense;
use anyhow::{Error, Result};
use sentry::integrations::anyhow::capture_anyhow;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use surf::http::Method;

/// Takes a Typesense query and URL, and fetches it via HTTP
/// Returns a Result with the query's output or an HTTP result
pub async fn handle_typesense<Q: Serialize, R: DeserializeOwned>(
	query: Q,
	url: &str,
	method: Method,
) -> Result<R, Error> {
	let request = match typesense().request(method, url).query(&query) {
		Ok(request) => request,
		Err(err) => {
			let error: Error = err.into_inner();
			handle_error(&error);
			return Err(error);
		}
	};

	let mut response = match request.send().await {
		Ok(response) => response,
		Err(err) => {
			let error: Error = err.into_inner();
			handle_error(&error);
			return Err(error);
		}
	};

	match response.status().is_success() {
		true => (),
		false => {
			let status = response.status();
			let body = response
				.body_string()
				.await
				.unwrap_or_else(|_| "".to_string());
			return Err(anyhow::anyhow!("{}: {}", status, body));
		}
	}

	let response = match response.body_json::<Value>().await {
		Ok(response) => response,
		Err(err) => {
			let error: Error = err.into_inner();
			handle_error(&error);
			return Err(error);
		}
	};

	let typed: R = match serde_json::from_value(response) {
		Ok(typed) => typed,
		Err(err) => {
			let error: Error = err.into();
			handle_error(&error);
			return Err(error);
		}
	};

	Ok(typed)
}

/// Takes an error and reports it to Sentry
pub fn handle_error(err: &Error) {
	let uuid = capture_anyhow(err);
	println!("--------------------------");
	println!("Reporting an error (Sentry UUID: {})", uuid);
	println!("Error: {}", err);
	if cfg!(debug_assertions) {
		println!("{:?}", err);
	}
	println!("--------------------------");
}
