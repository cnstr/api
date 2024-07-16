use chrono::Datelike;
use chrono::Utc;
use openapi::{generate_openapi, Metadata};
use serde::Deserialize;
use serde_json::to_string as to_string_json;
use serde_yaml::to_string as to_string_yaml;
use std::collections::HashMap;
use tokio::main;
use vergen::{vergen, Config, ShaKind};

#[warn(clippy::all)]
#[warn(clippy::correctness)]
#[warn(clippy::suspicious)]
#[warn(clippy::pedantic)]
#[warn(clippy::style)]
#[warn(clippy::complexity)]
#[warn(clippy::perf)]

/// Strongly-typed manifest
#[derive(Deserialize)]
pub struct Manifest {
	pub meta: Meta,
	pub build: Build,
	pub endpoints: Endpoints,
}

/// Strongly-typed meta section
#[derive(Deserialize)]
pub struct Meta {
	pub code_name: String,
	pub description: String,
	pub contact_email: String,
	pub production_name: String,
	pub copyright_string: String,
}

/// Strongly-typed build section
#[derive(Deserialize)]
pub struct Build {
	pub bump: Bump,
	pub piracy_endpoint: String,
	pub sentry_dsn: Conditional,
	pub k8s_control_plane: String,
	pub postgres_url: Conditional,
	pub typesense_host: Conditional,
	pub vector_host: Conditional,
}

#[derive(Deserialize)]
pub struct Conditional {
	pub debug: String,
	pub release: String,
}

#[derive(Deserialize)]
pub struct Bump {
	pub documentation_id: String,
	pub access_token: String,
}

/// Strongly-typed endpoints section
#[derive(Deserialize)]
pub struct Endpoints {
	pub api: String,
	pub docs: String,
	pub site: String,
	pub privacy: String,
	pub privacy_updated: String,
}

fn main() {
	register_vergen_envs();
	let copyright =
		env_or_die("CANISTER_META_COPYRIGHT").replace("{year}", &Utc::now().year().to_string());

	load_openapi(Metadata {
		name: env_or_die("CANISTER_META_NAME"),
		version: env!("CARGO_PKG_VERSION").to_string(),
		description: env_or_die("CANISTER_META_DESC"),
		contact: env_or_die("CANISTER_META_EMAIL"),
		license: copyright,
		endpoint: env_or_die("CANISTER_API_ENDPOINT"),
		cwd: "../openapi".to_string(),
	});
}

/// Registers environment variables from the 'vergen' crate
/// While unnecessary, the defaults are tuned to only what we want
fn register_vergen_envs() {
	let mut config = Config::default();

	// Disable default features we don't need
	*config.rustc_mut().sha_mut() = false;
	*config.git_mut().semver_mut() = false;
	*config.cargo_mut().enabled_mut() = false;
	*config.rustc_mut().channel_mut() = false;
	*config.sysinfo_mut().enabled_mut() = false;
	*config.rustc_mut().commit_date_mut() = false;
	*config.git_mut().commit_timestamp_mut() = false;

	// Reconfigure the Git SHA to be shortened output
	*config.git_mut().sha_kind_mut() = ShaKind::Short;

	match vergen(config) {
		Ok(_) => (),
		Err(e) => panic!("Failed to register 'vergen' configuration ({e})"),
	}
}

/// Set a cargo environment variable
/// Used for build-time variables
fn set_env(key: &str, value: &str) {
	println!("Registering environment variable: {key}={value}");
	println!("cargo:rustc-env={key}={value}");
}

/// Safely retrieves an environment variable or panics
/// Used for build-time variables
fn env_or_die(key: &str) -> String {
	match std::env::var(key) {
		Ok(value) => value,
		Err(_) => {
			eprintln!("FATAL: Missing Environment Variable: {}", key);
			std::process::exit(1);
		}
	}
}

/// Loads the OpenAPI schema via the 'openapi' crate
/// Sets the CANISTER_OPENAPI_YAML and CANISTER_OPENAPI_JSON environment variables
#[main]
async fn load_openapi(metadata: Metadata) {
	let api = generate_openapi(&metadata);

	let yaml = match to_string_yaml(&api) {
		Ok(yaml) => yaml.replace('\n', "\\n"),
		Err(e) => panic!("Failed to serialize OpenAPI YAML ({e})"),
	};

	set_env("CANISTER_OPENAPI_YAML", &yaml);

	let json = match to_string_json(&api) {
		Ok(json) => json,
		Err(e) => panic!("Failed to serialize OpenAPI JSON ({e})"),
	};

	set_env("CANISTER_OPENAPI_JSON", &json);

	// Check if we are running in the docker build environment
	// If we are, make an upload to the documentation server
	if std::env::var("CANISTER_UPLOAD_OPENAPI").is_ok() {
		let id = env_or_die("CANISTER_OPENAPI_ID");
		let token = env_or_die("CANISTER_OPENAPI_TOKEN");

		let mut body = HashMap::new();
		body.insert("documentation", &id);
		body.insert("definition", &json);

		let client = reqwest::Client::new();
		let response = client
			.post("https://bump.sh/api/v1/versions")
			.header("Authorization", format!("Token {}", token))
			.json(&body)
			.send()
			.await;

		match response {
			Ok(response) => {
				if response.status().is_success() {
					println!("Successfully uploaded OpenAPI schema to documentation server");
				} else {
					println!("Failed to upload OpenAPI schema to documentation server");
				}
			}
			Err(e) => {
				println!("Failed to upload OpenAPI schema to documentation server ({e})");
			}
		}
	}
}
