use std::{
	fs::{read_to_string, write},
	path::Path,
};

use bollard::Docker;
use openapi::{dump_openapi, Metadata};
use reqwest::Client;
use serde_json::{json, Value};
use serde_yaml::{from_str as from_yaml_str, to_string as to_yaml_string};

#[tokio::main]
async fn main() {
	let image_tag = get_docker_tag().await;
	update_manifest(image_tag);
	update_bump().await;
}

async fn update_bump() {
	let manifest = load_manifest();
	let yaml = dump_openapi(&Metadata {
		name: manifest["meta"]["product_name"].as_str().unwrap(),
		version: env!("CARGO_PKG_VERSION"),
		description: manifest["meta"]["description"].as_str().unwrap(),
		contact: manifest["meta"]["contact_email"].as_str().unwrap(),
		license: manifest["meta"]["copyright_string"].as_str().unwrap(),
		endpoint: manifest["endpoints"]["api"].as_str().unwrap(),
	});

	let documentation_id = manifest["build"]["bump"]["documentation_id"]
		.as_str()
		.unwrap();
	let access_token = manifest["build"]["bump"]["access_token"].as_str().unwrap();

	let client = Client::new();
	let response = client
		.post("https://bump.sh/api/v1/versions")
		.header("Authorization", format!("Token {}", access_token))
		.json(&json!({
			"documentation": documentation_id,
			"definition": yaml
		}))
		.send()
		.await;

	match response {
		Ok(response) => {
			// check if status is 201 or 204
			match response.status().as_u16() {
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
			}
		}
		Err(err) => {
			println!("Failed to request bump.sh API ({})", err);
		}
	};
}

fn load_manifest() -> Value {
	let manifest = match read_to_string("../manifest.yaml") {
		Ok(manifest) => manifest,
		Err(err) => {
			panic!("Failed to read manifest.yaml ({})", err)
		}
	};

	let manifest: Value = from_yaml_str(&manifest).unwrap();
	return manifest;
}

fn update_manifest(image_tag: String) {
	let path = Path::new("./kubernetes/api.yaml");
	let raw_manifest = std::fs::read_to_string(path).unwrap();
	let kube_objects = raw_manifest.split("---").collect::<Vec<&str>>();
	let mut manifest: Value = from_yaml_str(kube_objects[1]).unwrap();

	manifest["spec"]["template"]["spec"]["containers"][0]["image"] = Value::String(image_tag);
	write(
		path,
		format!(
			"{}---\n{}",
			kube_objects[0],
			to_yaml_string(&manifest).unwrap()
		),
	)
	.unwrap();
}

async fn get_docker_tag() -> String {
	let docker = match Docker::connect_with_local_defaults() {
		Ok(docker) => docker,
		Err(e) => panic!("Error connecting to docker: {}", e),
	};

	let mut image_tag = format!("tale.me/canister/api:{}", env!("CARGO_PKG_VERSION"));
	println!("Pulling hash for image: {}", image_tag);

	let image_hash = match docker.inspect_image(&image_tag).await {
		Ok(image) => match image.id {
			Some(id) => id,
			None => panic!("Image not found: {}", image_tag),
		},
		Err(e) => panic!("Error inspecting image: {}", e),
	};

	println!("Image hash: {}", image_hash);
	image_tag.push_str("@");
	image_tag.push_str(&image_hash);
	return image_tag;
}