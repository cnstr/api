use anyhow::Result;
use reqwest::ClientBuilder;
use serde_json::{from_str, Value};
use std::io::Write;
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
	fetch_k8s_details();

	add_config("CANISTER_PRODUCT_NAME", "Canister");
	add_config("CANISTER_CODE_NAME", "cnstr");
	add_config("CANISTER_CONTACT_EMAIL", "support@canister.me");
	add_config("CANISTER_COPYRIGHT", "Aarnav Tale (c) {{year}}");

	add_config("CANISTER_API_ENDPOINT", "https://api.canister.me/v2");
	add_config("CANISTER_DOCS_ENDPOINT", "https://docs.canister.me");
	add_config("CANISTER_PRIVACY_ENDPOINT", "https://canister.me/privacy");
	add_config("CANISTER_SITE_ENDPOINT", "https://canister.me");

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

#[tokio::main]
async fn fetch_k8s_details() {
	let client = ClientBuilder::new()
		.danger_accept_invalid_certs(true)
		.build()
		.unwrap();

	let url = format!("https://{}/version", "k8s-ctl-plane.tale.me:6443");
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
