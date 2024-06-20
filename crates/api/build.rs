use openapi::{generate_openapi, Metadata};
use reqwest::ClientBuilder;
use serde::Deserialize;
use serde_json::{from_str as from_json, to_string as to_string_json};
use serde_yaml::{from_str as from_yaml, to_string as to_string_yaml};
use std::{
	fs::{canonicalize, read_to_string},
	path::Path,
};
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

/// Kubernetes HTTP Response
#[derive(Deserialize)]
struct K8sResponse {
	#[serde(rename = "gitVersion")]
	git_version: String,
	platform: String,
}

fn main() {
	register_vergen_envs();
	let manifest = load_manifest("../../manifest.yaml");

	set_env("CANISTER_PRODUCTION_NAME", &manifest.meta.production_name);
	set_env("CANISTER_PRIVACY_ENDPOINT", &manifest.endpoints.privacy);
	set_env("CANISTER_CONTACT_EMAIL", &manifest.meta.contact_email);
	set_env("CANISTER_COPYRIGHT", &manifest.meta.copyright_string);
	set_env("CANISTER_DOCS_ENDPOINT", &manifest.endpoints.docs);
	set_env("CANISTER_SITE_ENDPOINT", &manifest.endpoints.site);
	set_env("CANISTER_API_ENDPOINT", &manifest.endpoints.api);
	set_env("CANISTER_CODE_NAME", &manifest.meta.code_name);

	set_env(
		"CANISTER_PRIVACY_UPDATED",
		&manifest.endpoints.privacy_updated,
	);

	load_openapi(Metadata {
		name: manifest.meta.production_name,
		version: env!("CARGO_PKG_VERSION").to_string(),
		description: manifest.meta.description,
		contact: manifest.meta.contact_email,
		license: manifest.meta.copyright_string,
		endpoint: manifest.endpoints.api,
		cwd: "../openapi".to_string(),
	});

	load_sentry_dsn(manifest.build.sentry_dsn);
	load_k8s_info(manifest.build.k8s_control_plane);
	load_piracy_urls(&manifest.build.piracy_endpoint);
	load_database_urls(
		manifest.build.postgres_url,
		manifest.build.typesense_host,
		manifest.build.vector_host,
	);
}

/// Loads the manifest.yaml file and deserializes it
/// Panics if the file is not found or if it fails to deserialize
pub fn load_manifest(path: &str) -> Manifest {
	let manifest_path = Path::new(path);
	let manifest = match read_to_string(manifest_path) {
		Ok(manifest) => {
			match canonicalize(manifest_path) {
				Ok(path) => println!("cargo:rerun-if-changed={}", path.display()),
				Err(e) => panic!("Failed to canonicalize manifest path ({e})"),
			};

			match from_yaml(&manifest) {
				Ok(value) => {
					let value: Manifest = value;
					value
				}
				Err(e) => panic!("Failed to parse manifest.yaml ({e})"),
			}
		}
		Err(e) => {
			panic!("Failed to read manifest.yaml ({e})")
		}
	};

	manifest
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

/// Loads the OpenAPI schema via the 'openapi' crate
/// Sets the CANISTER_OPENAPI_YAML and CANISTER_OPENAPI_JSON environment variables
fn load_openapi(metadata: Metadata) {
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
}

/// Loads the Sentry DSN from the manifest
fn load_sentry_dsn(dsn: Conditional) {
	let sentry_dsn = match cfg!(debug_assertions) {
		true => &dsn.debug,
		false => &dsn.release,
	};

	set_env("CANISTER_SENTRY_DSN", sentry_dsn);
}

/// Fetches the Kubernetes version from the control plane
/// Sets the CANISTER_K8S_VERSION environment variable
#[main]
async fn load_k8s_info(k8s_host: String) {
	let client = match ClientBuilder::new()
		.danger_accept_invalid_certs(true)
		.build()
	{
		Ok(client) => client,
		Err(e) => panic!("Failed to build insecure HTTP client ({e})"),
	};

	let url = format!("https://{k8s_host}/version");
	let json = match client.get(url).send().await {
		Ok(response) => match response.text().await {
			Ok(value) => {
				let value: K8sResponse = match from_json(&value) {
					Ok(value) => value,
					Err(e) => panic!("Failed to deserialize Kubernetes HTTP response ({e})"),
				};

				value
			}
			Err(e) => panic!("Failed to parse Kubernetes HTTP response ({e})"),
		},
		Err(e) => panic!("Failed to fetch Kubernetes version ({e})"),
	};

	set_env(
		"CANISTER_K8S_VERSION",
		format!("k8s_{}-{}", &json.git_version, &json.platform).as_str(),
	);
}

/// Fetches the piracy URLs from the piracy endpoint
/// Sets the CANISTER_PIRACY_URLS environment variable
/// Viewable at github.com/cnstr/manifests
#[main]
async fn load_piracy_urls(json_endpoint: &str) {
	let response = match reqwest::get(json_endpoint).await {
		Ok(response) => response,
		Err(e) => panic!("Failed to fetch piracy URLs ({e})"),
	};

	let value = match response.text().await {
		Ok(value) => value,
		Err(e) => panic!("Failed to parse piracy URLs ({e})"),
	};

	set_env("CANISTER_PIRACY_URLS", &value);
}

/// Loads the databse connection strings from the build details
/// Sets variables for PostgreSQL, Typesense, and Vector
fn load_database_urls(postgres: Conditional, typesense: Conditional, vector: Conditional) {
	let postgres_url = match cfg!(debug_assertions) {
		true => &postgres.debug,
		false => &postgres.release,
	};

	set_env("CANISTER_POSTGRES_URL", postgres_url);

	let vector_url = match cfg!(debug_assertions) {
		true => &vector.debug,
		false => &vector.release,
	};

	set_env("CANISTER_VECTOR_URL", vector_url);

	let typesense_host = match cfg!(debug_assertions) {
		true => {
			let binding = typesense.debug.split('@').collect::<Vec<&str>>();
			binding
		}
		false => {
			let binding = typesense.release.split('@').collect::<Vec<&str>>();
			binding
		}
	};

	let key = match typesense_host.first() {
		Some(key) => key,
		None => panic!("Failed to parse Typesense key"),
	};

	let host = match typesense_host.get(1) {
		Some(host) => host,
		None => panic!("Failed to parse Typesense host"),
	};

	set_env("CANISTER_TYPESENSE_KEY", key);
	set_env("CANISTER_TYPESENSE_HOST", host);
}
