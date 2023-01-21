use lazy_static::lazy_static;
use prisma_client_rust::QueryError;
use serde::Deserialize;
use std::future::Future;
use tokio::runtime::{Builder, Runtime};

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
