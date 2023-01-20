use crate::utility::api_respond;
use serde_json::json;
use tide::{Request, Result};

pub async fn not_found(_req: Request<()>) -> Result {
	api_respond(404, json!({}))
}
