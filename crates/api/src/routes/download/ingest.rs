use crate::{
	helpers::{clients, responses},
	prisma::package,
	utility::{load_runtime_config, parse_user_agent},
};
use axum::{
	http::{HeaderMap, HeaderValue, StatusCode},
	response::IntoResponse,
	Json,
};
use chrono::Utc;
use once_cell::sync::OnceCell;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::to_value;
use std::sync::OnceLock;

#[derive(Debug, Serialize, Deserialize)]
struct BadRequest {
	status: String,
	date: String,
	error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
	pub package_id: String,
	pub package_version: String,
	pub package_author: Option<String>,
	pub package_maintainer: Option<String>,
	pub repository_uri: String,
	pub repository_suite: Option<String>,
	pub repository_component: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DownloadEvent {
	package_id: String,
	package_version: String,
	package_author: String,
	package_maintainer: String,
	repository_uri: String,
	repository_suite: String,
	repository_component: String,

	client: String,
	client_version: String,
	jailbreak: String,
	jailbreak_version: String,
	distribution: String,
	distribution_version: String,

	client_architecture: String,
	client_bitness: String,
	device: String,
	device_platform: String,
	device_version: String,

	database_uuid: Option<String>,
	time: i64,
}

static HTTP: OnceCell<Client> = OnceCell::new();
static VECTOR_URL: OnceLock<String> = OnceLock::new();

fn try_get_header(header: Option<&HeaderValue>) -> String {
	match header {
		Some(header) => match header.to_str() {
			Ok(header) => header.to_string(),
			Err(_) => "unknown".to_string(),
		},
		None => "unknown".to_string(),
	}
}

pub async fn ingest(headers: HeaderMap, body: Option<Json<Vec<Payload>>>) -> impl IntoResponse {
	let body = match body {
		Some(body) => body,
		None => return responses::error(StatusCode::BAD_REQUEST, "Invalid request body"),
	};

	let user_agent = match headers.get("Sec-CH-UA") {
		Some(user_agent) => match user_agent.to_str() {
			Ok(user_agent) => parse_user_agent(user_agent),
			Err(_) => return responses::error(StatusCode::BAD_REQUEST, "Invalid user agent"),
		},
		None => return responses::error(StatusCode::BAD_REQUEST, "Missing user agent"),
	};

	let architecture = try_get_header(headers.get("Sec-CH-UA-Arch"));
	let bitness = try_get_header(headers.get("Sec-CH-UA-Bitness"));
	let model = try_get_header(headers.get("Sec-CH-UA-Model"));
	let platform = try_get_header(headers.get("Sec-CH-UA-Platform"));
	let platform_version = try_get_header(headers.get("Sec-CH-UA-Platform-Version"));

	let (client, client_version) = match user_agent.iter().find(|brand| brand.r#type == "client") {
		Some(brand) => (brand.name.clone(), brand.version.clone()),
		None => ("unknown".to_string(), "unknown".to_string()),
	};

	let (jailbreak, jailbreak_version) =
		match user_agent.iter().find(|brand| brand.r#type == "jailbreak") {
			Some(brand) => (brand.name.clone(), brand.version.clone()),
			None => ("unknown".to_string(), "unknown".to_string()),
		};

	let (distribution, distribution_version) = match user_agent
		.iter()
		.find(|brand| brand.r#type == "distribution")
	{
		Some(brand) => (brand.name.clone(), brand.version.clone()),
		None => ("unknown".to_string(), "unknown".to_string()),
	};

	let mut events: Vec<DownloadEvent> = vec![];

	for entry in body.iter() {
		let package_id = &entry.package_id.to_string();
		let package_version = &entry.package_version.to_string();

		let package_author = match &entry.package_author {
			Some(package_author) => package_author.to_string(),
			None => "none".to_string(),
		};

		let package_maintainer = match &entry.package_maintainer {
			Some(package_maintainer) => package_maintainer.to_string(),
			None => "none".to_string(),
		};

		let repository_uri = &entry.repository_uri.to_string();
		let repository_suite = match &entry.repository_suite {
			Some(repository_suite) => repository_suite.to_string(),
			None => "./".to_string(),
		};

		let repository_component = match &entry.repository_component {
			Some(repository_component) => repository_component.to_string(),
			None => "none".to_string(),
		};

		let database_uuid = match clients::prisma(|prisma| {
			prisma
				.package()
				.find_first(vec![
					package::package::equals(package_id.to_string()),
					package::version::equals(package_version.to_string()),
					package::is_pruned::equals(false),
				])
				.with(package::repository::fetch())
				.exec()
		})
		.await
		{
			Ok(package_search) => package_search.map(|package_search| package_search.uuid),
			Err(_) => None,
		};

		let event = DownloadEvent {
			package_id: package_id.to_string(),
			package_version: package_version.to_string(),
			package_author,
			package_maintainer,
			repository_uri: repository_uri.to_string(),
			repository_suite,
			repository_component,

			client: client.clone(),
			client_version: client_version.clone(),
			jailbreak: jailbreak.clone(),
			jailbreak_version: jailbreak_version.clone(),
			distribution: distribution.clone(),
			distribution_version: distribution_version.clone(),
			client_architecture: architecture.clone(),
			client_bitness: bitness.clone(),
			device: model.clone(),
			device_platform: platform.clone(),
			device_version: platform_version.clone(),

			database_uuid,
			time: Utc::now().timestamp(),
		};

		events.push(event);
	}

	let return_value = match to_value(events) {
		Ok(return_value) => return_value,
		Err(_) => {
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to serialize events",
			);
		}
	};

	let http_client = match HTTP.get_or_try_init(|| {
		Client::builder()
			.timeout(std::time::Duration::from_secs(5))
			.build()
	}) {
		Ok(http_client) => http_client,
		Err(_) => {
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to initialize HTTP client",
			);
		}
	};

	let vector_url = VECTOR_URL.get_or_init(|| {
		let config = load_runtime_config();
		config.vector_url
	});

	let cloned_payload = return_value.clone();
	let response = http_client
		.post(vector_url)
		.json(&cloned_payload)
		.send()
		.await;

	match response {
		Ok(_) => (),
		Err(_) => {
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to send event to ingest",
			);
		}
	}

	responses::data(StatusCode::OK, return_value)
}

// TODO: Implement
pub async fn ingest_healthy() -> bool {
	true
}
