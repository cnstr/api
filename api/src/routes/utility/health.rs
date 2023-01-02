use serde_json::{json, to_string_pretty};
use tide::{Request, Result};

pub async fn health(_req: Request<()>) -> Result {
	return Ok(to_string_pretty(&json!({
		"status": "OK"
	}))
	.unwrap()
	.into());
}
