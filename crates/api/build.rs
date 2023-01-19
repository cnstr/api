use manifest::{load_manifest, Database};
use openapi::{generate_openapi, Metadata};
use reqwest::ClientBuilder;
use serde::Deserialize;
use serde_json::{from_str as from_json, to_string as to_string_json};
use serde_yaml::to_string as to_string_yaml;
use tokio::main;
use vergen::{vergen, Config, ShaKind};

#[warn(clippy::all)]
#[warn(clippy::correctness)]
#[warn(clippy::suspicious)]
#[warn(clippy::pedantic)]
#[warn(clippy::style)]
#[warn(clippy::complexity)]
#[warn(clippy::perf)]

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

	load_openapi(Metadata {
		name: manifest.meta.production_name,
		version: env!("CARGO_PKG_VERSION").to_string(),
		description: manifest.meta.description,
		contact: manifest.meta.contact_email,
		license: manifest.meta.copyright_string,
		endpoint: manifest.endpoints.api,
		cwd: "../openapi".to_string(),
	});

	load_k8s_info(manifest.build.k8s_control_plane);
	load_piracy_urls(&manifest.build.piracy_endpoint);
	load_database_urls(manifest.build.postgres_url, manifest.build.typesense_host);
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
		Ok(yaml) => yaml.replace("\n", "\\n"),
		Err(e) => panic!("Failed to serialize OpenAPI YAML ({e})"),
	};

	set_env("CANISTER_OPENAPI_YAML", &yaml);

	let json = match to_string_json(&api) {
		Ok(json) => json,
		Err(e) => panic!("Failed to serialize OpenAPI JSON ({e})"),
	};

	set_env("CANISTER_OPENAPI_JSON", &json);
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
/// Sets the CANISTER_POSTGRES_URL and CANISTER_ELASTIC_URL environment variables
fn load_database_urls(postgres: Database, elastic: Database) {
	let postgres_url = match cfg!(debug_assertions) {
		true => &postgres.debug,
		false => &postgres.release,
	};

	set_env("CANISTER_POSTGRES_URL", postgres_url);

	let typesense_host = match cfg!(debug_assertions) {
		true => {
			let binding = elastic.debug.split("@").collect::<Vec<&str>>();
			binding
		}
		false => {
			let binding = elastic.release.split("@").collect::<Vec<&str>>();
			binding
		}
	};

	let key = match typesense_host.get(0) {
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
