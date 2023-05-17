use anyhow::Error;
use sentry::integrations::anyhow::capture_anyhow;

/// Takes an error and reports it to Sentry
pub fn handle_error(err: &Error) {
	let uuid = capture_anyhow(err);
	println!("--------------------------");
	println!("Reporting an error (Sentry UUID: {})", uuid);
	println!("Error: {}", err);
	if cfg!(debug_assertions) {
		println!("{:?}", err);
	}
	println!("--------------------------");
}
