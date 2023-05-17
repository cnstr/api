use sentry::capture_error;
use std::error::Error;

pub async fn report_error(error: impl Error) {
	// Only send to Sentry in production
	if cfg!(not(debug_assertions)) {
		let uuid = capture_error(&error);
		println!("error: reported to sentry with uuid: {}", uuid);
	}

	println!("error: {:?}", error);
}
