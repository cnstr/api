use std::collections::HashMap;

use serde_json::{json, Map, Value};

pub fn generate_schema(options: Value) -> Value {
	let schema_name = match options["schema_name"].is_string() {
		true => options["schema_name"].as_str().unwrap(),
		false => panic!("Attempting to generate schema without a schema_name"),
	};

	let mut schema = match options["schema"].is_object() {
		true => options["schema"].as_object().unwrap().to_owned(),
		false => panic!("Failed to generate {} (missing schema)", schema_name),
	};

	let mut descriptions = match options["descriptions"].is_object() {
		true => options["descriptions"].as_object().unwrap().to_owned(),
		false => panic!("Failed to generate {} (missing descriptions)", schema_name),
	};

	let nullables = match options["nullables"].is_array() {
		true => options["nullables"]
			.as_array()
			.unwrap()
			.iter()
			.map(|key| key.as_str().unwrap().to_string())
			.collect::<Vec<String>>(),
		false => Vec::<String>::new(),
	};

	let schema = translate_schema(&mut schema, &mut descriptions, &nullables);
	return json!({ schema_name: schema });

	// return recursive_records(&mut schema, &options);
}

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
					"description": descriptions.get(key).unwrap(),
					"nullable": nullables.contains(key),
					"items": {
						"type": "string"
					}
				}),
			);

			continue;
		}

		if value.is_object() {
			let mut sub_object = value.as_object().unwrap().to_owned();
			let mut sub_descriptions = descriptions
				.get(key)
				.unwrap()
				.as_object()
				.unwrap()
				.to_owned();

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
				"description": descriptions.get(key).unwrap(),
			}),
		);
	}

	return json!({
		"type": "object",
		"properties": openapi_properties,
	});
}

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
