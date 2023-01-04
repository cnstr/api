use anyhow::Result;
use openapi::{build_openapi, Metadata};
use reqwest::ClientBuilder;
use serde_json::{from_str, Value};
use serde_yaml::from_str as from_yaml;
use std::{
	fs::read_to_string,
	io::{stdout, Write},
};
use vergen::{vergen, Config, ShaKind};

fn main() -> Result<()> {
	// VERGEN_BUILD_TIMESTAMP
	// VERGEN_BUILD_SEMVER

	// VERGEN_GIT_BRANCH
	// VERGEN_GIT_SHA_SHORT

	// VERGEN_RUSTC_HOST_TRIPLE
	// VERGEN_RUSTC_LLVM_VERSION
	// VERGEN_RUSTC_SEMVER

	let mut config = Config::default();
	*config.git_mut().sha_kind_mut() = ShaKind::Short;

	vergen(config)?;
	let manifest = load_manifest();
	fetch_k8s_details(manifest["build"]["k8s_control_plane"].as_str().unwrap());
	fetch_piracy_urls(manifest["build"]["piracy_endpoint"].as_str().unwrap());
	set_database_urls(manifest["build"].clone());

	// CANISTER_OPENAPI_YAML
	build_openapi(&Metadata {
		name: manifest["meta"]["product_name"].as_str().unwrap(),
		version: env!("CARGO_PKG_VERSION"),
		description: manifest["meta"]["description"].as_str().unwrap(),
		contact: manifest["meta"]["contact_email"].as_str().unwrap(),
		license: manifest["meta"]["copyright_string"].as_str().unwrap(),
		endpoint: manifest["endpoints"]["api"].as_str().unwrap(),
	});
	return Ok(());
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

fn is_debug() -> bool {
	return std::env::var("PROFILE").unwrap() == "debug";
}

fn load_manifest() -> Value {
	let manifest = match read_to_string("../manifest.yaml") {
		Ok(manifest) => manifest,
		Err(err) => {
			panic!("Failed to read manifest.yaml ({})", err)
		}
	};

	let manifest: Value = from_yaml(&manifest).unwrap();

	add_config(
		"CANISTER_PRODUCT_NAME",
		manifest["meta"]["product_name"].as_str().unwrap(),
	);

	add_config(
		"CANISTER_CODE_NAME",
		manifest["meta"]["code_name"].as_str().unwrap(),
	);

	add_config(
		"CANISTER_CONTACT_EMAIL",
		manifest["meta"]["contact_email"].as_str().unwrap(),
	);

	add_config(
		"CANISTER_COPYRIGHT",
		manifest["meta"]["copyright_string"].as_str().unwrap(),
	);

	add_config(
		"CANISTER_API_ENDPOINT",
		manifest["endpoints"]["api"].as_str().unwrap(),
	);

	add_config(
		"CANISTER_DOCS_ENDPOINT",
		manifest["endpoints"]["docs"].as_str().unwrap(),
	);

	add_config(
		"CANISTER_PRIVACY_ENDPOINT",
		manifest["endpoints"]["privacy"].as_str().unwrap(),
	);

	add_config(
		"CANISTER_SITE_ENDPOINT",
		manifest["endpoints"]["site"].as_str().unwrap(),
	);

	return manifest;
}

#[tokio::main]
async fn fetch_k8s_details(control_plane_host: &str) {
	let client = ClientBuilder::new()
		.danger_accept_invalid_certs(true)
		.build()
		.unwrap();

	let url = format!("https://{}/version", control_plane_host);
	let response = client.get(url).send().await.unwrap();
	let value: Value = from_str(&response.text().await.unwrap()).unwrap();

	add_config(
		"CANISTER_K8S_VERSION",
		format!(
			"k8s_{}-{}",
			value["gitVersion"].as_str().unwrap(),
			value["platform"].as_str().unwrap()
		)
		.as_str(),
	);
}

#[tokio::main]
async fn fetch_piracy_urls(json_endpoint: &str) {
	let response = reqwest::get(json_endpoint).await.unwrap();
	let value = response.text().await.unwrap();
	add_config("CANISTER_PIRACY_URLS", &value);
}

fn set_database_urls(value: Value) {
	let postgres_url = match is_debug() {
		true => value["postgres_url"]["debug"].as_str().unwrap(),
		false => value["postgres_url"]["release"].as_str().unwrap(),
	};

	add_config("CANISTER_POSTGRES_URL", postgres_url);

	let elastic_url = match is_debug() {
		true => value["elastic_url"]["debug"].as_str().unwrap(),
		false => value["elastic_url"]["release"].as_str().unwrap(),
	};

	add_config("CANISTER_ELASTIC_URL", elastic_url);
}
