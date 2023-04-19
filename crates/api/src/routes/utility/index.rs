use std::env;

use crate::utility::http_respond;
use chrono::{Datelike, Utc};
use serde_json::json;
use tide::{Request, Result};

/// Returns the landing page for the Canister API
pub async fn index(req: Request<()>) -> Result {
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

	let copyright = env!("CANISTER_COPYRIGHT").replace("{{year}}", &Utc::now().year().to_string());
	let current_date = Utc::now().date_naive().to_string();
	let current_epoch = Utc::now().timestamp();

	let remote_address = req.remote().unwrap_or("Unknown");
	let user_agent = match req.header("User-Agent") {
		Some(user_agent) => user_agent.as_str(),
		None => "Unknown",
	};

	let pod_name = match env::var("POD_NAME") {
		Ok(pod_name) => pod_name,
		Err(_) => "unknown".to_string(),
	};

	http_respond(
		200,
		json!({
			"info": {
				"name": name,
				"version": env!("VERGEN_BUILD_SEMVER"),
				"build": build,
				"platform": platform,
				"runtime": env!("CANISTER_K8S_VERSION"),
				"server_origin": pod_name,
			},

			"reference": {
				"docs": env!("CANISTER_DOCS_ENDPOINT"),
				"privacy_policy": env!("CANISTER_PRIVACY_ENDPOINT"),
				"contact_email": env!("CANISTER_CONTACT_EMAIL"),
				"copyright": copyright,
			},

			"connection": {
				"current_date": current_date,
				"current_epoch": current_epoch,
				"remote_address": remote_address,
				"user_agent": user_agent,
			}
		}),
	)
}
