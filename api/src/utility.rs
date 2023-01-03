use serde::Serialize;
use serde_json::{ser::PrettyFormatter, to_value, Serializer, Value};
use tide::StatusCode;

pub fn merge_json<T>(serial: T, json: Value) -> Value
where
	T: Serialize,
{
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
