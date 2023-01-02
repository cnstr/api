use serde_json::{json, to_string_pretty};
use tide::{Request, Result};

pub async fn index(req: Request<()>) -> Result {
	return Ok(to_string_pretty(&json!({
		"info": {
			"name": "Canister (cnstr)",
			"version": env!("VERGEN_BUILD_SEMVER"),
			"build": format!("{} (git-{}-tree/{})", env!("VERGEN_BUILD_TIMESTAMP"), env!("VERGEN_GIT_SHA_SHORT"), env!("VERGEN_GIT_BRANCH")),
			"platform": format!("rust-{} ({}_llvm{})", env!("VERGEN_RUSTC_SEMVER"), env!("VERGEN_RUSTC_HOST_TRIPLE"), env!("VERGEN_RUSTC_LLVM_VERSION"))
		},

		"reference": {
			"docs": "https://docs.canister.me",
			"privacy_policy": "https://canister.me/privacy",
			"contact_email": "support@canister.me",
			"copyright": "Aarnav Tale (c) 2022"
		},

		"connection": {
			"current_date": chrono::Utc::now().date_naive().to_string(),
			"current_epoch": chrono::Utc::now().timestamp(),
			"remote_address": req.remote().unwrap_or("Unknown").to_string(),
			"user_agent": req.header("User-Agent").unwrap()[0].to_string()
		}
	}))
	.unwrap()
	.into());
}
