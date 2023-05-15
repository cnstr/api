use axum::{
	http::{header, StatusCode},
	response::IntoResponse,
};

pub async fn openapi_json() -> impl IntoResponse {
	let body = env!("CANISTER_OPENAPI_JSON");

	(
		StatusCode::NOT_FOUND,
		[(header::CONTENT_TYPE, "application/json")],
		body,
	)
}
