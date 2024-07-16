use super::{handle_error, load_runtime_config};
use serde::Serialize;
use serde_json::{to_value, Value};
use std::sync::OnceLock;
use url::Url;

static API_ENDPOINT: OnceLock<String> = OnceLock::new();

/// Merges two JSON objects together in the order of left, right
/// If the object is a strictly-typed struct, it is serialized into a Value
pub fn merge_json<L: Serialize, R: Serialize>(left: L, right: R) -> Value {
	let mut left = match to_value(left) {
		Ok(value) => value,
		Err(err) => {
			handle_error(&err.into());
			return Value::Null;
		}
	};

	let right = match to_value(right) {
		Ok(value) => value,
		Err(err) => {
			handle_error(&err.into());
			return Value::Null;
		}
	};

	merge_json_value(&mut left, right);
	left
}

// Modified to be a bit more readable and concise
// Original: https://stackoverflow.com/questions/47070876/how-can-i-merge-two-json-objects-with-rust
fn merge_json_value(left: &mut Value, right: Value) {
	match (left, right) {
		(&mut Value::Object(ref mut left), Value::Object(right)) => {
			for (key, value) in right {
				merge_json_value(left.entry(key).or_insert(Value::Null), value);
			}
		}

		(left, right) => *left = right,
	}
}

/// Generates pagination links with the given URL path and page number
/// The next parameter determines if this is the last page or not
pub fn page_links(path: &str, page: u8, next: bool) -> (Option<String>, Option<String>) {
	let endpoint = API_ENDPOINT.get_or_init(|| {
		let config = load_runtime_config();
		config.api_endpoint
	});

	let url = format!("{}{}", endpoint, path);
	let mut url = match Url::parse(&url) {
		Ok(url) => url,
		Err(err) => {
			handle_error(&err.into());
			return (None, None);
		}
	};

	let prev_page = match page > 1 {
		true => Some(
			url.clone()
				.query_pairs_mut()
				.append_pair("page", (page - 1).to_string().as_str())
				.finish()
				.as_str()
				.to_owned(),
		),
		false => None,
	};

	let next_page = match next {
		true => Some(
			url.query_pairs_mut()
				.append_pair("page", (page + 1).to_string().as_str())
				.finish()
				.as_str()
				.to_owned(),
		),
		false => None,
	};

	(prev_page, next_page)
}

pub fn api_endpoint() -> String {
	API_ENDPOINT
		.get_or_init(|| {
			let config = load_runtime_config();
			config.api_endpoint
		})
		.clone()
}
