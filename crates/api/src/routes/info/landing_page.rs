use crate::helpers::responses;
use axum::{http::StatusCode, response::IntoResponse};
use chrono::Datelike;
use chrono::Utc;
use serde_json::json;
use std::env::var;

pub async fn landing_page() -> impl IntoResponse {
	let name = format!(
		"{} ({})",
		env!("CANISTER_PRODUCTION_NAME"),
		env!("CANISTER_CODE_NAME")
	);

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

	let runtime = format!(
		"{}-{}",
		env!("CANISTER_K8S_VERSION"),
		var("POD_NAME").unwrap_or("unknown".to_string())
	);

	let copyright = env!("CANISTER_COPYRIGHT").replace("{{year}}", &Utc::now().year().to_string());

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
				"docs": env!("CANISTER_DOCS_ENDPOINT"),
				"privacy_policy": env!("CANISTER_PRIVACY_ENDPOINT"),
				"privacy_updated": env!("CANISTER_PRIVACY_UPDATED"),
				"contact_email": env!("CANISTER_CONTACT_EMAIL"),
				"copyright": copyright,
			},
		}),
	)
}
