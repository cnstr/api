use crate::{
	prisma::package,
	utility::{api_respond, error_respond, handle_async, handle_prisma, parse_user_agent, prisma},
};
use chrono::Utc;
use once_cell::sync::OnceCell;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_value};
use surf::http::headers::HeaderValues;
use tide::{Request, Result};

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

fn try_get_header(header: Option<&HeaderValues>) -> String {
	match header {
		Some(header) => header.as_str().to_string(),
		None => "unknown".to_string(),
	}
}

pub async fn download_ingest(mut req: Request<()>) -> Result {
	let body = match req.body_json::<Payload>().await {
		Ok(body) => body,
		Err(_) => return error_respond(400, "Invalid request body"),
	};

	let user_agent = match req.header("Sec-CH-UA") {
		Some(user_agent) => parse_user_agent(user_agent.as_str()),
		None => return error_respond(400, "Missing user agent"),
	};

	let architecture = try_get_header(req.header("Sec-CH-UA-Arch"));
	let bitness = try_get_header(req.header("Sec-CH-UA-Bitness"));
	let model = try_get_header(req.header("Sec-CH-UA-Model"));
	let platform = try_get_header(req.header("Sec-CH-UA-Platform"));
	let platform_version = try_get_header(req.header("Sec-CH-UA-Platform-Version"));

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

	let package_id = &body.package_id.to_string();
	let package_version = &body.package_version.to_string();

	let package_author = match &body.package_author {
		Some(package_author) => package_author.to_string(),
		None => "none".to_string(),
	};

	let package_maintainer = match &body.package_maintainer {
		Some(package_maintainer) => package_maintainer.to_string(),
		None => "none".to_string(),
	};

	let repository_uri = &body.repository_uri.to_string();
	let repository_suite = match &body.repository_suite {
		Some(repository_suite) => repository_suite.to_string(),
		None => "./".to_string(),
	};

	let repository_component = match &body.repository_component {
		Some(repository_component) => repository_component.to_string(),
		None => "none".to_string(),
	};

	let database_uuid = match handle_prisma(
		prisma()
			.package()
			.find_first(vec![
				package::package::equals(package_id.to_string()),
				package::version::equals(package_version.to_string()),
				package::is_pruned::equals(false),
			])
			.with(package::repository::fetch())
			.exec(),
	) {
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

		client,
		client_version,
		jailbreak,
		jailbreak_version,
		distribution,
		distribution_version,
		client_architecture: architecture,
		client_bitness: bitness,
		device: model,
		device_platform: platform,
		device_version: platform_version,

		database_uuid,
		time: Utc::now().timestamp(),
	};

	let return_value = match to_value(event) {
		Ok(return_value) => return_value,
		Err(_) => {
			return error_respond(500, "Failed to serialize event");
		}
	};

	let http_client = match HTTP.get_or_try_init(|| {
		Client::builder()
			.timeout(std::time::Duration::from_secs(5))
			.build()
	}) {
		Ok(http_client) => http_client,
		Err(_) => {
			return error_respond(500, "Failed to initialize HTTP client");
		}
	};

	let cloned_payload = return_value.clone();
	let response = handle_async(async move {
		http_client
			.post("http://localhost:8687/")
			.json(&cloned_payload)
			.send()
			.await
	});

	match response {
		Ok(_) => (),
		Err(_) => {
			return error_respond(500, "Failed to send event to ingest");
		}
	}

	api_respond(
		200,
		json!({
			"event": return_value,
		}),
	)
}

// TODO: Implement
pub async fn download_ingest_healthy() -> bool {
	true
}
