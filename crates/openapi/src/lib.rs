use chrono::{Datelike, Utc};
use schema::{generate_schema, Schema};
use serde::Serialize;
use serde_json::{Map, Value};
use serde_yaml::from_str;
use std::{
	fs::{read_dir, read_to_string},
	path::Path,
};

mod schema;

#[warn(clippy::all)]
#[warn(clippy::correctness)]
#[warn(clippy::suspicious)]
#[warn(clippy::pedantic)]
#[warn(clippy::style)]
#[warn(clippy::complexity)]
#[warn(clippy::perf)]

/// Metadatata needed to generate OpenAPI
pub struct Metadata {
	pub name: String,
	pub version: String,
	pub description: String,
	pub contact: String,
	pub license: String,
	pub endpoint: String,
	pub cwd: String,
}

/// Strongly-typed OpenAPI schema
#[derive(Serialize)]
pub struct OpenAPI {
	pub openapi: String,
	pub info: Info,
	pub servers: Vec<Server>,
	pub paths: Value,
	pub components: Components,
}

/// Strongly-typed info section
#[derive(Serialize)]
pub struct Info {
	pub title: String,
	pub version: String,
	pub description: String,
	pub contact: Contact,
	pub license: License,
}

/// Strongly-typed contact section
#[derive(Serialize)]
pub struct Contact {
	pub name: String,
	pub email: String,
}

/// Strongly-typed license section
#[derive(Serialize)]
pub struct License {
	pub name: String,
}

/// Strongly-typed server section
#[derive(Serialize)]
pub struct Server {
	pub url: String,
	pub description: String,
}

/// Strongly-typed components section
#[derive(Serialize)]
pub struct Components {
	pub schemas: Value,
}

/// Generates the OpenAPI schema with the given Metadata
/// Returns it it as a strongly-typed struct for further processing
pub fn generate_openapi(meta: &Metadata) -> OpenAPI {
	OpenAPI {
		openapi: "3.0.0".to_string(),
		info: Info {
			title: meta.name.clone(),
			version: meta.version.clone(),
			description: meta.description.clone(),
			contact: Contact {
				name: "Aarnav Tale".to_string(),
				email: meta.contact.clone(),
			},
			license: License {
				name: meta
					.license
					.replace("{{year}}", &Utc::now().year().to_string()),
			},
		},
		servers: vec![Server {
			url: meta.endpoint.clone(),
			description: "Production API".to_string(),
		}],
		paths: generate_routes(&meta.cwd),
		components: Components {
			schemas: generate_schemas(&meta.cwd),
		},
	}
}

/// Reads schemas, populates descriptions, and returns them as a Value
/// This is used to populate the components section of the OpenAPI schema
fn generate_schemas(cwd: &str) -> Value {
	let joined_cwd = format!("{}/schemas", cwd);
	let path = Path::new(&joined_cwd);

	let files = match read_dir(path) {
		Ok(files) => files,
		Err(err) => {
			panic!("Failed to read schemas directory ({})", err)
		}
	};

	let schemas = files
		.map(|file| {
			let file = match file {
				Ok(file) => file,
				Err(err) => {
					panic!("Failed to read schema ({})", err)
				}
			};

			let file_name = match file.file_name().into_string() {
				Ok(file_name) => file_name,
				Err(err) => {
					panic!("Failed to read schema file ({:?})", err)
				}
			};

			let file = match read_to_string(file.path()) {
				Ok(file) => file,
				Err(err) => {
					panic!("Failed to open schemas/{} ({})", file_name, err)
				}
			};

			let contents = file;
			let value: Schema = from_str(&contents).unwrap();
			let schema = generate_schema(value);
			return schema;
		})
		.collect::<Vec<Value>>();

	return Value::Object({
		let mut map = Map::new();
		for schema in schemas {
			let schema = match schema.as_object() {
				Some(schema) => schema,
				None => {
					panic!("Failed to parse schema")
				}
			};

			for (key, value) in schema {
				map.insert(key.to_string(), value.clone());
			}
		}
		map
	});
}

/// Reads routes, populates descriptions, and returns them as a Value
/// This is used to populate the paths section of the OpenAPI schema
fn generate_routes(cwd: &str) -> Value {
	let joined_cwd = format!("{}/routes", cwd);
	let path = Path::new(&joined_cwd);

	let files = match read_dir(path) {
		Ok(files) => files,
		Err(err) => {
			panic!("Failed to read routes directory ({})", err)
		}
	};

	let routes = files
		.map(|file| {
			let file = match file {
				Ok(file) => file,
				Err(err) => {
					panic!("Failed to read route ({})", err)
				}
			};

			let file_name = match file.file_name().into_string() {
				Ok(file_name) => file_name,
				Err(err) => {
					panic!("Failed to read route file ({:?})", err)
				}
			};

			let file = match read_to_string(file.path()) {
				Ok(file) => file,
				Err(err) => {
					panic!("Failed to open routes/{} ({})", file_name, err)
				}
			};

			let contents = file;
			return contents;
		})
		.collect::<Vec<String>>()
		.join("\n");

	match from_str(&routes) {
		Ok(combined) => {
			let value: Value = combined;
			return value;
		}
		Err(err) => {
			panic!("Failed to parse routes ({})", err)
		}
	};
}
