use std::{
	fs::{read_dir, read_to_string},
	io::{stdout, Write},
	path::Path,
};

use chrono::Datelike;
use schema::generate_schema;
use serde_json::{from_str as from_json_str, json, to_string as to_json_string, Value};
use serde_yaml::{from_str as from_yaml_str, to_string as to_yaml_string};

mod schema;

pub struct Metadata<'a> {
	pub name: &'a str,
	pub version: &'a str,
	pub description: &'a str,
	pub contact: &'a str,
	pub license: &'a str,
	pub endpoint: &'a str,
}

// Working directory is /api/
pub fn build_openapi(meta: &Metadata) {
	let openapi = json!({
		"openapi": "3.0.0",
		"info": {
			"title": meta.name,
			"version": meta.version,
			"description": meta.description,
			"contact": {
				"name": "Aarnav Tale",
				"email": meta.contact,
			},
			"license": {
				"name": meta.license.replace("{{year}}", &chrono::Utc::now().year().to_string()),
			},
		},
		"servers": [
			{
				"url": meta.endpoint,
				"description": "Production API",
			},
		],
		"paths": read_manifests("../openapi/routes"),
		"components": {
			"schemas": read_schemas("../openapi/schemas")
		}
	});

	let openapi_yaml = to_yaml_string(&openapi).unwrap();
	add_config("CANISTER_OPENAPI_YAML", &openapi_yaml.replace("\n", "\\n"));

	let openapi_json = to_json_string(&openapi).unwrap();
	add_config("CANISTER_OPENAPI_JSON", &openapi_json);
}

pub fn dump_openapi(meta: &Metadata) -> String {
	let openapi = json!({
		"openapi": "3.0.0",
		"info": {
			"title": meta.name,
			"version": meta.version,
			"description": meta.description,
			"contact": {
				"name": "Aarnav Tale",
				"email": meta.contact,
			},
			"license": {
				"name": meta.license.replace("{{year}}", &chrono::Utc::now().year().to_string()),
			},
		},
		"servers": [
			{
				"url": meta.endpoint,
				"description": "Production API",
			},
		],
		"paths": read_manifests("./openapi/routes"),
		"components": {
			"schemas": read_schemas("./openapi/schemas")
		}
	});

	let openapi_json = to_json_string(&openapi).unwrap();
	return openapi_json;
}

fn read_schemas(folder: &str) -> Value {
	let path = Path::new(folder);
	let mut openapi_files = Vec::<String>::new();

	let files = match read_dir(path) {
		Ok(files) => files,
		Err(err) => {
			panic!("Failed to read schemas directory ({})", err)
		}
	};

	for file in files {
		let file = file.unwrap();
		let file_name = file.file_name().into_string().unwrap();

		if file.path().is_file() && file_name.ends_with(".json") {
			let file = match read_to_string(file.path()) {
				Ok(file) => file,
				Err(err) => {
					panic!("Failed to open schemas/{} ({})", file_name, err)
				}
			};

			let contents = file;
			openapi_files.push(contents);
		}
	}

	let openapi_schemas = openapi_files
		.iter()
		.map(|file| {
			let value: Value = from_json_str(file).unwrap();
			let schema = generate_schema(value);
			return schema;
		})
		.collect::<Vec<Value>>();

	return Value::Object({
		let mut map = serde_json::Map::new();
		for schema in openapi_schemas {
			let schema = schema.as_object().unwrap();
			for (key, value) in schema {
				map.insert(key.to_string(), value.clone());
			}
		}
		map
	});
}

fn read_manifests(folder: &str) -> Value {
	let path = Path::new(folder);
	let mut openapi_files = Vec::<String>::new();

	let folders = match read_dir(path) {
		Ok(files) => files,
		Err(err) => {
			panic!("Failed to read routes directory ({})", err)
		}
	};

	for folder in folders {
		let folder = folder.unwrap();

		if folder.path().is_dir() {
			let files = match read_dir(folder.path()) {
				Ok(files) => files,
				Err(err) => {
					panic!(
						"Failed to read routes/{} directory ({})",
						folder.file_name().to_str().unwrap(),
						err
					)
				}
			};

			for file in files {
				let file = file.unwrap();
				let file_name = file.file_name().into_string().unwrap();

				if file.path().is_file() && file_name.ends_with(".yaml") {
					let file = match read_to_string(file.path()) {
						Ok(file) => file,
						Err(err) => {
							panic!(
								"Failed to open routes/{}/{} ({})",
								folder.file_name().to_str().unwrap(),
								file_name,
								err
							)
						}
					};

					let contents = file;
					openapi_files.push(contents);
				}
			}
		}
	}

	let value: Value = from_yaml_str(&openapi_files.join("\n")).unwrap();
	return value;
}

fn add_config(key: &str, value: &str) {
	let stdout = &mut stdout();
	match writeln!(stdout, "cargo:rustc-env={}={}", key, value) {
		Ok(_) => {}
		Err(err) => {
			panic!("Failed to configure config-key: {} ({})", key, err)
		}
	}
}
