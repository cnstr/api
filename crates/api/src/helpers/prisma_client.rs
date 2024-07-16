use crate::{helpers::report_error, prisma::PrismaClient, utility::load_runtime_config};
use once_cell::sync::OnceCell;
use prisma_client_rust::QueryError;
use sentry::{capture_message, Level};
use serde::de::DeserializeOwned;
use std::{future::Future, process::exit};

static PRISMA_CLIENT: OnceCell<PrismaClient> = OnceCell::new();

async fn prisma_client() -> &'static PrismaClient {
	let global_client = PRISMA_CLIENT.get();
	let config = load_runtime_config();

	if global_client.is_none() {
		let client = match PrismaClient::_builder()
			.with_url(config.database_url)
			.build()
			.await
		{
			Ok(client) => client,
			Err(err) => {
				capture_message("failed to create prisma client", Level::Fatal);
				println!("panic: failed to create prisma client: {}", err);
				exit(1);
			}
		};

		match PRISMA_CLIENT.set(client) {
			Ok(client) => client,
			Err(_) => {
				capture_message("failed to create prisma client", Level::Fatal);
				println!("panic: failed to create prisma client");
				exit(1);
			}
		};

		return PRISMA_CLIENT.get().unwrap();
	}

	global_client.unwrap()
}

pub async fn prisma<T: DeserializeOwned, F: Future<Output = Result<T, QueryError>>>(
	callback: impl FnOnce(&'static PrismaClient) -> F,
) -> Result<T, QueryError> {
	match callback(prisma_client().await).await {
		Ok(result) => Ok(result),
		Err(err) => {
			report_error(&err).await;
			Err(err)
		}
	}
}
