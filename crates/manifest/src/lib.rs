use serde::Deserialize;
use serde_yaml::from_str as from_yaml;
use std::{
	fs::{canonicalize, read_to_string},
	path::Path,
};

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
