use super::{handle_error, merge_json};
use anyhow::Error;
use chrono::Utc;
use http::StatusCode;
use serde_json::{json, to_string_pretty, Value};
use tide::{Response, Result};

/// Returns a response with the given status code and body
fn respond(status_code: u16, mut body: Value, should_merge: bool) -> Result {
	let status_verb = match StatusCode::from_u16(status_code) {
		Ok(status) => status.canonical_reason().unwrap_or("Unknown"),
		Err(_) => "Unknown",
	};

	if should_merge {
		body = merge_json(
			json!({
				"message": format!("{status_code} {status_verb}"),
				"date": Utc::now().to_rfc3339()
			}),
			body,
		);
	}

	let body = match to_string_pretty(&body) {
		Ok(body) => body,
		Err(err) => {
			let anyhow: Error = err.into();
			handle_error(&anyhow);
			return Err(anyhow.into());
		}
	};

	Ok(Response::builder(status_code)
		.header("Content-Type", "application/json")
		.body(body)
		.build())
}

/// Returns a response with the given status code and body
pub fn http_respond(status_code: u16, body: Value) -> Result {
	respond(status_code, body, false)
}

/// Returns a response with the given status code and body
/// The body is merged with a date and status message
pub fn api_respond(status_code: u16, body: Value) -> Result {
	respond(status_code, body, true)
}

/// Returns a response with the given status code and error message
pub fn error_respond(status_code: u16, message: &str) -> Result {
	api_respond(status_code, json!({ "error": message }))
}
