use std::future::Future;

use lazy_static::lazy_static;
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
