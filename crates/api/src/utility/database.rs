use super::handle_error;
use crate::prisma::PrismaClient;
use anyhow::Error;
use once_cell::sync::OnceCell;

static PRISMA: OnceCell<PrismaClient> = OnceCell::new();

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

/// Returns the globalized Prisma Client
pub fn prisma() -> &'static PrismaClient {
	match PRISMA.get() {
		Some(client) => client,
		None => panic!("Prisma Client not initialized"),
	}
}
