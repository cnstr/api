use axum::{http::StatusCode, Json};
use chrono::Utc;
use serde::Serialize;
use serde_json::{json, Value};

type Response = (StatusCode, Json<Value>);

pub fn data<T: Serialize>(status_code: StatusCode, body: T) -> Response {
	let response_status = format!(
		"{} {}",
		status_code.as_u16(),
		// If there is no status reason, don't show it
		status_code.canonical_reason().unwrap_or("")
	);

	let body = json!({
		"status": response_status,
		"date": Utc::now().to_rfc3339(),
		"data": body
	});

	(status_code, Json(body))
}

pub fn error<T: Serialize>(status_code: StatusCode, body: T) -> Response {
	let response_status = format!(
		"{} {}",
		status_code.as_u16(),
		// If there is no status reason, don't show it
		status_code.canonical_reason().unwrap_or("")
	);

	let body = json!({
		"status": response_status,
		"date": Utc::now().to_rfc3339(),
		"error": body
	});

	(status_code, Json(body))
}
