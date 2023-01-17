use std::collections::HashMap;

use serde::Deserialize;
use serde_json::{json, Map, Value};

/// Strongly-typed OpenAPI Component schema
#[derive(Deserialize)]
pub struct Schema {
	pub schema_name: String,
	pub schema: Map<String, Value>,
	pub descriptions: Map<String, Value>,
	pub nullables: Option<Vec<String>>,
}

/// Generates the OpenAPI compliant schema from the provided options
/// Recursively transverses the object to handle nested objects and arrays
pub fn generate_schema(mut options: Schema) -> Value {
	let nullables = match options.nullables {
		Some(options) => options,
		None => Vec::<String>::new(),
	};

	let schema = translate_schema(&mut options.schema, &mut options.descriptions, &nullables);
	return json!({ options.schema_name: schema });
}

/// Translates the provided schema into an OpenAPI compliant schema
/// This is the recursive function that handles nested objects and arrays
fn translate_schema(
	schema: &mut Map<String, Value>,
	descriptions: &mut Map<String, Value>,
	nullables: &Vec<String>,
) -> Value {
	let mut openapi_properties = HashMap::<&str, Value>::new();

	for (key, value) in schema {
		if value.is_array() {
			openapi_properties.insert(
				key,
				json!({
					"type": "array",
					"description": match descriptions.get(key) {
						Some(description) => description,
						None => panic!("Missing description for {}", key),
					},
					"nullable": nullables.contains(key),
					"items": {
						"type": "string"
					}
				}),
			);

			continue;
		}

		if value.is_object() {
			let mut sub_object = match value.as_object() {
				Some(object) => object.to_owned(),
				None => panic!("Failed to convert {} to object", key),
			};

			let mut sub_descriptions = match descriptions.get(key) {
				Some(description) => match description.as_object() {
					Some(object) => object.to_owned(),
					None => panic!("Failed to convert {} to object", key),
				},

				None => panic!("Missing description for {}", key),
			};

			openapi_properties.insert(
				key,
				translate_schema(&mut sub_object, &mut sub_descriptions, nullables),
			);
			continue;
		}

		if value.is_null() {
			continue;
		}

		openapi_properties.insert(
			key,
			json!({
				"type": get_type(value),
				"example": value,
				"nullable": nullables.contains(&key),
				"description": match descriptions.get(key) {
					Some(description) => description,
					None => panic!("Missing description for {}", key),
				},
			}),
		);
	}

	return json!({
		"type": "object",
		"properties": openapi_properties,
	});
}

/// Returns the OpenAPI type for the provided value
fn get_type(value: &mut Value) -> &'static str {
	if value.is_array() {
		return "array";
	}

	if value.is_object() {
		return "object";
	}

	if value.is_null() {
		return "null";
	}

	if value.is_string() {
		return "string";
	}

	if value.is_number() {
		return "number";
	}

	if value.is_boolean() {
		return "boolean";
	}

	return "unknown";
}
