use chrono::Datelike;
use serde_json::json;
use tide::{Request, Result, StatusCode::Ok as OK};

use crate::utility::json_respond;

pub async fn index(req: Request<()>) -> Result {
	return Ok(json_respond(
		OK,
		json!({
			"info": {
				"name": format!("{} ({})", env!("CANISTER_PRODUCT_NAME"), env!("CANISTER_CODE_NAME")),
				"version": env!("VERGEN_BUILD_SEMVER"),
				"build": format!("{}+git-{}-tree/{}", env!("VERGEN_BUILD_TIMESTAMP"), env!("VERGEN_GIT_SHA_SHORT"), env!("VERGEN_GIT_BRANCH")),
				"platform": format!("rust-{}+{}_llvm{}", env!("VERGEN_RUSTC_SEMVER"), env!("VERGEN_RUSTC_HOST_TRIPLE"), env!("VERGEN_RUSTC_LLVM_VERSION")),
				"runtime": env!("CANISTER_K8S_VERSION")
			},

			"reference": {
				"docs": env!("CANISTER_DOCS_ENDPOINT"),
				"privacy_policy": env!("CANISTER_PRIVACY_ENDPOINT"),
				"contact_email": env!("CANISTER_CONTACT_EMAIL"),
				"copyright": env!("CANISTER_COPYRIGHT").replace("{{year}}", &chrono::Utc::now().year().to_string())
			},

			"connection": {
				"current_date": chrono::Utc::now().date_naive().to_string(),
				"current_epoch": chrono::Utc::now().timestamp(),
				"remote_address": req.remote().unwrap_or("Unknown").to_string(),
				"user_agent": req.header("User-Agent").unwrap()[0].to_string()
			}
		}),
	));
}
