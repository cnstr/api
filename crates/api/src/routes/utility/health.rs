use serde_json::json;
use tide::{Request, Result, StatusCode::Ok as OK};

use crate::utility::json_respond;

pub async fn health(_req: Request<()>) -> Result {
	return Ok(json_respond(
		OK,
		json!({
			"status": "OK"
		}),
	));
}
