use chrono::Utc;
use http::StatusCode;
use serde_json::{json, to_string_pretty, Value};
use tide::{Response, Result};

use super::merge_json;

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
			// TODO: Sentry Error
			println!("Failed to stringify JSON response body");
			return Err(err.into());
		}
	};

	Ok(Response::builder(status_code)
		.header("Content-Type", "application/json")
		.body(body)
		.build())
}

pub fn http_respond(status_code: u16, body: Value) -> Result {
	respond(status_code, body, false)
}

pub fn api_respond(status_code: u16, body: Value) -> Result {
	respond(status_code, body, true)
}
