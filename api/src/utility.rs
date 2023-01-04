use std::future::Future;

use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, to_value, Serializer, Value};
use tide::StatusCode;
use tokio::runtime::{Builder, Runtime};
use url::Url;

pub fn merge_json<T: Serialize>(serial: T, json: Value) -> Value {
	let mut serial = to_value(serial).unwrap();
	merge_json_value(&mut serial, json);
	serial
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
	let formatter = PrettyFormatter::with_indent(b"\t");
	let mut serialized = Serializer::with_formatter(buffer, formatter);

	value.serialize(&mut serialized).unwrap();
	String::from_utf8(serialized.into_inner()).unwrap()
}

pub fn json_respond(status: StatusCode, value: Value) -> tide::Response {
	tide::Response::builder(status)
		.header("Content-Type", "application/json")
		.body(json_stringify(value))
		.build()
}

pub fn page_links(url: &str, page: u8, next: bool) -> (Option<String>, Option<String>) {
	let base = Url::parse(env!("CANISTER_API_ENDPOINT")).unwrap();
	let url = base.join(url).unwrap();

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
