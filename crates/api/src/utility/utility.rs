use std::future::Future;

use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, to_value, Serializer, Value};
use tokio::runtime::{Builder, Runtime};
use url::Url;

pub fn merge_json<L: Serialize, R: Serialize>(left: L, right: R) -> Value {
	let mut left = to_value(left).unwrap();
	let right = to_value(right).unwrap();

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

pub fn json_stringify(value: Value) -> String {
	let buffer = Vec::new();
	let formatter = PrettyFormatter::with_indent(b"    ");
	let mut serialized = Serializer::with_formatter(buffer, formatter);

	value.serialize(&mut serialized).unwrap();
	String::from_utf8(serialized.into_inner()).unwrap()
}

pub fn json_respond(status: tide::StatusCode, value: Value) -> tide::Response {
	tide::Response::builder(status)
		.header("Content-Type", "application/json")
		.body(json_stringify(value))
		.build()
}

/// Generates pagination links with the given URL path and page number
/// The next parameter determines if this is the last page or not
pub fn page_links(path: &str, page: u8, next: bool) -> (Option<String>, Option<String>) {
	let url = format!("{}{}", env!("CANISTER_API_ENDPOINT"), path);

	let url = match Url::parse(&url) {
		Ok(url) => url,
		Err(_) => {
			// TODO: Sentry Error
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
			url.clone()
				.query_pairs_mut()
				.append_pair("page", (page + 1).to_string().as_str())
				.finish()
				.as_str()
				.to_owned(),
		),
		false => None,
	};

	(prev_page, next_page)
}

lazy_static! {
	static ref RUNTIME: Runtime = Builder::new_multi_thread().enable_all().build().unwrap();
}

pub fn tokio_run<F: Future>(future: F) -> <F as Future>::Output {
	return RUNTIME.block_on(future);
}
