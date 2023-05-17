use axum::{
	http::{header, StatusCode},
	response::IntoResponse,
};

pub async fn openapi_yaml() -> impl IntoResponse {
	let body = env!("CANISTER_OPENAPI_YAML").replace("\\n", "\n");
	(StatusCode::OK, [(header::CONTENT_TYPE, "text/yaml")], body)
}
