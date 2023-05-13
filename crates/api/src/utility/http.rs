use std::env;

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

	let pod_name = match env::var("POD_NAME") {
		Ok(pod_name) => pod_name,
		Err(_) => "unknown".to_string(),
	};

	Ok(Response::builder(status_code)
		.header("Content-Type", "application/json")
		.header("X-Server-Origin", pod_name)
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

#[derive(Debug)]
pub struct Brand {
	pub name: String,
	pub r#type: String,
	pub version: String,
}

fn parse_brand(input: &str) -> Option<Brand> {
	let mut name = String::new();
	let mut r#type = String::new();
	let mut version = String::new();

	for item in input.split(';') {
		let kv_pair: Vec<&str> = item.split('=').collect();

		match kv_pair.len() {
			// Brand name doesn't follow the key=value format
			1 => name = kv_pair[0].to_string(),
			2 => match kv_pair[0] {
				"t" => r#type = kv_pair[1].to_string(),
				"v" => version = kv_pair[1].to_string(),
				_ => (),
			},
			_ => (),
		}
	}

	if name.is_empty() {
		return None;
	}

	if r#type.is_empty() {
		r#type = "unknown".to_string();
	}

	if version.is_empty() {
		version = "unknown".to_string();
	}

	Some(Brand {
		name,
		r#type,
		version,
	})
}

pub fn parse_user_agent(input: &str) -> Vec<Brand> {
	let mut brands = Vec::new();

	for item in input.split(',') {
		if let Some(brand) = parse_brand(item) {
			brands.push(brand);
		}
	}

	brands
}
