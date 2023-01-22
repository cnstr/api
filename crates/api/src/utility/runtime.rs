use lazy_static::lazy_static;
use prisma_client_rust::QueryError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::future::Future;
use surf::http::Method;
use tide::Result as HttpResult;
use tokio::runtime::{Builder, Runtime};

use super::{error_respond, typesense};

lazy_static! {
	static ref RUNTIME: Runtime = match Builder::new_multi_thread().enable_all().build() {
		Ok(runtime) => runtime,
		Err(e) => panic!("Failed to create Tokio thread for async runtime {e}"),
	};
}

/// Runs a future on the Tokio runtime thread
pub fn handle_async<F: Future>(future: F) -> F::Output {
	return RUNTIME.block_on(future);
}

/// Takes a Prisma query and executes it on the Tokio runtime thread
/// Returns a Result with the query's output or an HTTP result
pub fn handle_prisma<'a, T: Deserialize<'a>, F: Future<Output = Result<T, QueryError>>>(
	query: F,
) -> Result<T, QueryError> {
	match handle_async(query) {
		Ok(result) => Ok(result),
		Err(err) => Err(err),
	}
}

pub async fn handle_typesense<Q: Serialize, R: DeserializeOwned>(
	query: Q,
	url: &str,
	method: Method,
) -> Result<R, HttpResult> {
	let request = match typesense().request(method, url).query(&query) {
		Ok(request) => request,
		Err(err) => {
			// TODO: Sentry Handler
			println!("Failed to create Typesense query: {}", err);
			return Err(error_respond(500, "Failed to create Typesense query"));
		}
	};

	let response = match typesense().recv_json::<R>(request).await {
		Ok(response) => response,
		Err(err) => {
			println!("Failed to execute Typesense query: {}", err);
			return Err(error_respond(500, "Failed to execute Typesense query"));
		}
	};

	Ok(response)
}
