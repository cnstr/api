use serde_json::json;
use tide::{Request, Result, StatusCode::Ok as OK};

use crate::utility::json_respond;

pub async fn not_found(_req: Request<()>) -> Result {
	Ok(json_respond(
		OK,
		json!({
			"status": "404 Not Found",
			"date": chrono::Utc::now().to_rfc3339(),
		}),
	))
}
