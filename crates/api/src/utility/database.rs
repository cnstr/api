use super::handle_error;
use crate::prisma::PrismaClient;
use anyhow::Error;
use once_cell::sync::OnceCell;
use surf::{Client, Config, Url};

static PRISMA: OnceCell<PrismaClient> = OnceCell::new();
static TYPESENSE: OnceCell<Client> = OnceCell::new();

/// Connects to the Prisma client and globalizes it
pub async fn create_prisma_client() {
	let client = match PrismaClient::_builder()
		.with_url(env!("CANISTER_POSTGRES_URL").to_string())
		.build()
		.await
	{
		Ok(client) => client,
		Err(err) => {
			let anyhow: Error = err.into();
			handle_error(&anyhow);
			panic!("Failed to create Prisma Client: {}", anyhow)
		}
	};

	match PRISMA.set(client) {
		Ok(_) => (),
		Err(_) => panic!("Failed to globalize Prisma Client"),
	}
}

/// Connects to the Typesense client and globalizes it
pub fn create_typesense_client() {
	let url = format!("http://{}:8108", env!("CANISTER_TYPESENSE_HOST"));
	let base_url = match Url::parse(&url) {
		Ok(url) => url,
		Err(err) => {
			let anyhow: Error = err.into();
			handle_error(&anyhow);
			panic!("Failed to parse Typesense Host: {}", anyhow)
		}
	};

	let client = match Config::new()
		.set_base_url(base_url)
		.add_header("X-Typesense-API-Key", env!("CANISTER_TYPESENSE_KEY"))
	{
		Ok(client) => {
			let client: Client = match client.try_into() {
				Ok(client) => client,
				Err(err) => {
					let anyhow: Error = err.into();
					handle_error(&anyhow);
					panic!("Failed to create Typesense Client: {}", anyhow)
				}
			};

			client
		}
		Err(err) => {
			let anyhow: Error = err.into_inner();
			handle_error(&anyhow);
			panic!("Failed to create Typesense Client: {}", anyhow)
		}
	};

	match TYPESENSE.set(client) {
		Ok(_) => (),
		Err(_) => panic!("Failed to globalize Typesense Client"),
	}
}

/// Returns the globalized Prisma Client
pub fn prisma() -> &'static PrismaClient {
	match PRISMA.get() {
		Some(client) => client,
		None => panic!("Prisma Client not initialized"),
	}
}

/// Returns the globalized Typesense Client
pub fn typesense() -> &'static Client {
	match TYPESENSE.get() {
		Some(client) => client,
		None => panic!("Typesense Client not initialized"),
	}
}
