#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
	pub id: String,
	pub aliases: Option<Vec<String>>,
	pub visible: bool,
	pub quality: i32,
	pub package_count: i64,
	pub sections: Vec<String>,
	pub bootstrap: bool,
	pub uri: String,
	pub suite: String,
	pub component: Option<String>,
	pub name: Option<String>,
	pub version: Option<String>,
	pub description: Option<String>,
	pub date: Option<String>,
	pub payment_gateway: Option<String>,
	pub sileo_endpoint: Option<String>,

	pub origin_hostname: String,
	pub origin_release_path: String,
	pub origin_release_hash: String,
	pub origin_packages_path: String,
	pub origin_packages_hash: String,
	pub origin_last_updated: String,
	pub origin_has_in_release: bool,
	pub origin_has_release_gpg: bool,
	pub origin_supports_payment_v1: bool,
	pub origin_supports_payment_v2: bool,
	pub origin_uses_https: bool,

	// Old fields that are grandfathered in
	pub slug: String,
	pub tier: i32,
	pub isBootstrap: bool,
}
