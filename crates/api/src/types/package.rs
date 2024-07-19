#![allow(non_snake_case)]
use super::Repository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
	pub id: String,
	pub package_id: String,
	pub latest_version: bool,
	pub visible: bool,
	pub quality: i32,
	pub repository_id: String,
	pub price: String,
	pub version: String,
	pub architecture: String,
	pub package_filename: String,
	pub package_size: i64,
	pub sha256_hash: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub author: Option<String>,
	pub maintainer: Option<String>,
	pub depiction: Option<String>,
	pub native_depiction: Option<String>,
	pub sileo_depiction: Option<String>,
	pub header_url: Option<String>,
	pub tint_color: Option<String>,
	pub icon_url: Option<String>,
	pub section: Option<String>,
	pub tags: Option<Vec<String>>,
	pub installed_size: Option<i64>,

	// Old fields that are grandfathered in
	pub package: String,
	pub repositoryTier: i32,
	pub sileoDepiction: Option<String>,
	pub repository: Repository,
}
