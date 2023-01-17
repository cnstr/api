use manifest::{load_manifest, Database};
use openapi::{build_openapi, Metadata};
use reqwest::ClientBuilder;
use serde::Deserialize;
use serde_json::from_str as from_json;
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
	let manifest = load_manifest();

	set_env("CANISTER_PRODUCTION_NAME", &manifest.meta.production_name);
	set_env("CANISTER_PRIVACY_ENDPOINT", &manifest.endpoints.privacy);
	set_env("CANISTER_CONTACT_EMAIL", &manifest.meta.contact_email);
	set_env("CANISTER_COPYRIGHT", &manifest.meta.copyright_string);
	set_env("CANISTER_DOCS_ENDPOINT", &manifest.endpoints.docs);
	set_env("CANISTER_SITE_ENDPOINT", &manifest.endpoints.site);
	set_env("CANISTER_API_ENDPOINT", &manifest.endpoints.api);
	set_env("CANISTER_CODE_NAME", &manifest.meta.code_name);

	load_k8s_info(manifest.build.k8s_control_plane);
	load_piracy_urls(&manifest.build.piracy_endpoint);
	load_database_urls(manifest.build.postgres_url, manifest.build.elastic_url);

	// CANISTER_OPENAPI_YAML
	build_openapi(&Metadata {
		name: &manifest.meta.production_name,
		version: env!("CARGO_PKG_VERSION"),
		description: &manifest.meta.description,
		contact: &manifest.meta.contact_email,
		license: &manifest.meta.copyright_string,
		endpoint: &manifest.endpoints.api,
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

	let elastic_url = match cfg!(debug_assertions) {
		true => &elastic.debug,
		false => &elastic.release,
	};

	set_env("CANISTER_ELASTIC_URL", elastic_url);
}
