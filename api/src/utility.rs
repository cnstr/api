use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};
use tide::StatusCode;

// Modified to be a bit more readable and concise
// Original: https://stackoverflow.com/questions/47070876/how-can-i-merge-two-json-objects-with-rust
pub fn merge_json(left: &mut Value, right: Value) {
	match (left, right) {
		(left @ &mut Value::Object(_), Value::Object(right)) => {
			let left = left.as_object_mut().unwrap();
			for (key, value) in right {
				merge_json(left.entry(key).or_insert(Value::Null), value);
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
