use crate::helpers::responses;
use crate::utility::load_runtime_config;
use axum::{http::StatusCode, response::IntoResponse};
use chrono::Datelike;
use chrono::Utc;
use serde_json::json;
use std::env::var;

pub async fn landing_page() -> impl IntoResponse {
	let config = load_runtime_config();

	let name = format!("{} ({})", config.meta_name, config.meta_code);

	let build = format!(
		"{}+git-{}-tree/{}",
		env!("VERGEN_BUILD_TIMESTAMP"),
		env!("VERGEN_GIT_SHA_SHORT"),
		env!("VERGEN_GIT_BRANCH")
	);

	let platform = format!(
		"rust-{}+{}_llvm{}",
		env!("VERGEN_RUSTC_SEMVER"),
		env!("VERGEN_RUSTC_HOST_TRIPLE"),
		env!("VERGEN_RUSTC_LLVM_VERSION")
	);

	let runtime = format!("k8s-{}", var("POD_NAME").unwrap_or("unknown".to_string()));
	let copyright = config
		.meta_copyright
		.replace("{year}", &Utc::now().year().to_string());

	responses::data(
		StatusCode::OK,
		json!({
			"info": {
				"name": name,
				"version": env!("VERGEN_BUILD_SEMVER"),
				"build": build,
				"platform": platform,
				"runtime": runtime,
			},

			"reference": {
				"docs": config.docs_endpoint,
				"privacy_policy": config.privacy_endpoint,
				"privacy_updated": config.privacy_updated,
				"contact_email": config.meta_email,
				"copyright": copyright,
			},
		}),
	)
}
