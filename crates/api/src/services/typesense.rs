use crate::{helpers::responses, utility::load_runtime_config};
use anyhow::Result;
use axum::Json;
use reqwest::{header::HeaderMap, redirect::Policy, Method, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{process::exit, sync::OnceLock};

type Response = (StatusCode, Json<Value>);
static TYPESENSE: OnceLock<Client> = OnceLock::new();

pub struct Client {
	inner: reqwest::Client,
	base_url: String,
}

impl Client {
	pub fn get(&self, method: Method, url: &str) -> RequestBuilder {
		self.inner
			.request(method, format!("{}/{}", self.base_url, url))
	}

	pub async fn query<T: DeserializeOwned>(
		&self,
		query: &impl Serialize,
		path: &str,
	) -> Result<T, Response> {
		let request = self.get(Method::GET, path).query(&query);

		let response = match request.send().await {
			Ok(response) => response,
			Err(e) => {
				eprintln!("[indexer] failed to query typesense: {}", e);
				return Err(responses::error(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Failed to query internal search engine",
				));
			}
		};

		let status = response.status();
		if status.as_u16() >= 400 {
			let body = match response.text().await {
				Ok(body) => body,
				Err(e) => {
					eprintln!("[indexer] failed to read typesense response: {}", e);
					return Err(responses::error(
						StatusCode::INTERNAL_SERVER_ERROR,
						"Invalid response from internal search engine",
					));
				}
			};

			eprintln!("[indexer] typesense error: {}: {}", status, body);
			return Err(responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Internal search engine error",
			));
		}

		let body = match response.json::<T>().await {
			Ok(body) => body,
			Err(e) => {
				eprintln!("[indexer] failed to parse typesense response: {}", e);
				return Err(responses::error(
					StatusCode::INTERNAL_SERVER_ERROR,
					"Response from internal search engine is invalid",
				));
			}
		};

		Ok(body)
	}

	pub async fn connect(&self) -> Result<()> {
		let client = create_ts();

		for i in 1..=10 {
			match client.get(Method::GET, "health").send().await {
				Ok(response) => {
					if response.status().is_success() {
						println!("[indexer] connected to typesense after {} attempts", i);
						return Ok(());
					}

					if i == 10 {
						return Err(response.error_for_status().unwrap_err().into());
					}
				}
				Err(e) => {
					eprintln!("[indexer] failed to connect to typesense: {}", e);
					if i == 10 {
						return Err(e.into());
					}
				}
			}

			tokio::time::sleep(std::time::Duration::from_secs(1)).await;
		}

		return Err(anyhow::anyhow!(
			"failed to connect to typesense after 10 attempts"
		));
	}
}

pub fn create_ts() -> &'static Client {
	let config = load_runtime_config();

	TYPESENSE.get_or_init(|| {
		let mut headers = HeaderMap::new();
		headers.insert(
			"X-Typesense-API-Key",
			config.typesense_api_key.parse().unwrap(),
		);

		let inner = match reqwest::Client::builder()
			.default_headers(headers)
			.timeout(std::time::Duration::from_secs(30))
			.redirect(Policy::limited(15))
			.build()
		{
			Ok(client) => client,
			Err(e) => {
				eprintln!("[indexer] failed to create typesense client: {}", e);
				// TODO: Sentry fatal
				exit(1);
			}
		};

		Client {
			inner,
			base_url: config.typesense_url.trim_end_matches('/').to_string(),
		}
	})
}
