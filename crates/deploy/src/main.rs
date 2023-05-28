use manifest::load_manifest;
use openapi::{generate_openapi, Metadata};
use reqwest::Client;
use serde_json::{json, to_string as to_json_string, Value};
use serde_yaml::{from_str as from_yaml_str, to_string as to_yaml_string};
use std::{
	env,
	fs::{read_to_string, write},
	path::Path,
};
use tokio::main;

#[warn(clippy::all)]
#[warn(clippy::correctness)]
#[warn(clippy::suspicious)]
#[warn(clippy::pedantic)]
#[warn(clippy::style)]
#[warn(clippy::complexity)]
#[warn(clippy::perf)]

fn main() {
	// Don't run if not in CI
	if env::var("CI").is_err() {
		println!("Not in CI, skipping deployment...");
		return;
	}

	update_kubernetes_manifest();
	publish_openapi();
}

/// Sends a POST request to bump.sh to update the OpenAPI documentation
/// The changes are automatically deployed to docs.canister.me
#[main]
async fn publish_openapi() {
	let manifest = load_manifest("./manifest.yaml");
	let api = generate_openapi(&Metadata {
		name: manifest.meta.production_name,
		version: env!("CARGO_PKG_VERSION").to_string(),
		description: manifest.meta.description,
		contact: manifest.meta.contact_email,
		license: manifest.meta.copyright_string,
		endpoint: manifest.endpoints.api,
		cwd: "./crates/openapi".to_string(),
	});

	let openapi_yaml = match to_yaml_string(&api) {
		Ok(yaml) => yaml,
		Err(err) => {
			println!("Failed to serialize OpenAPI to YAML ({})", err);
			return;
		}
	};

	let http = Client::new();
	let body = match to_json_string(&json!({
		"documentation": manifest.build.bump.documentation_id,
		"definition": openapi_yaml
	})) {
		Ok(body) => body,
		Err(err) => {
			println!("Failed to build bump.sh request body ({})", err);
			return;
		}
	};

	let response = http
		.post("https://bump.sh/api/v1/versions")
		.header(
			"Authorization",
			format!("Token {}", manifest.build.bump.access_token),
		)
		.header("Content-Type", "application/json")
		.body(body)
		.send()
		.await;

	match response {
		Ok(response) => match response.status().as_u16() {
			201 => {
				println!("Successfully updated bump.sh documentation");
			}

			204 => {
				println!("Already updated bump.sh documentation");
			}

			_ => {
				println!(
					"Failed to update bump.sh documentation ({})",
					response.status()
				);
			}
		},
		Err(err) => {
			println!("Failed to request bump.sh API ({})", err);
		}
	};
}

/// Modifies the Kubernetes manifest to use the latest image tag
/// This is done by parsing the manifest as YAML and modifying the image tag
fn update_kubernetes_manifest() {
	let image = format!(
		"us-east4-docker.pkg.dev/aarnavtale/canister/api:{}",
		env!("CARGO_PKG_VERSION")
	);

	let path = Path::new("./kubernetes/api.yaml");
	let raw_manifest = match read_to_string(path) {
		Ok(manifest) => manifest,
		Err(err) => {
			println!("Failed to read Kubernetes manifest ({})", err);
			return;
		}
	};

	let kube_objects = raw_manifest.split("---").collect::<Vec<&str>>();
	let mut manifest: Value = match from_yaml_str(kube_objects[1]) {
		Ok(manifest) => manifest,
		Err(err) => {
			println!("Failed to parse Kubernetes manifest ({})", err);
			return;
		}
	};

	manifest["spec"]["template"]["spec"]["containers"][0]["image"] = Value::String(image);
	match write(
		path,
		format!(
			"{}---\n{}",
			kube_objects[0],
			match to_yaml_string(&manifest) {
				Ok(yaml) => yaml,
				Err(err) => {
					println!("Failed to serialize Kubernetes manifest ({})", err);
					return;
				}
			}
		),
	) {
		Ok(_) => {
			println!("Successfully updated Kubernetes manifest");
		}
		Err(err) => {
			println!("Failed to write Kubernetes manifest ({})", err);
		}
	}
}
