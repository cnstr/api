use anyhow::Result;
use reqwest::ClientBuilder;
use serde_json::{from_str, Value};
use serde_yaml::from_str as from_yaml;
use std::{fs::read_to_string, io::Write};
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
	let build_credentials = load_manifest();
	fetch_k8s_details(build_credentials["k8s_control_plane"].as_str().unwrap());

	return Ok(());
}

fn add_config(key: &str, value: &str) {
	let stdout = &mut std::io::stdout();
	match writeln!(stdout, "cargo:rustc-env={}={}", key, value) {
		Ok(_) => {}
		Err(err) => {
			panic!("Failed to configure config-key: {} ({})", key, err)
		}
	}
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

	return manifest["build"].clone();
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
