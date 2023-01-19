pub use crate::prisma;

use self::prisma::PrismaClient;
use once_cell::sync::OnceCell;
use surf::{Client, Config, Url};

static PRISMA: OnceCell<PrismaClient> = OnceCell::new();
static TYPESENSE: OnceCell<Client> = OnceCell::new();

pub async fn create_prisma() {
	let client = PrismaClient::_builder()
		.with_url(env!("CANISTER_POSTGRES_URL").to_string())
		.build()
		.await;

	match client {
		Ok(client) => PRISMA.set(client).unwrap(),
		Err(err) => panic!("Failed to connect to database: {}", err),
	}
}

pub fn create_typesense_client() {
	let url = format!("http://{}:8108", env!("CANISTER_TYPESENSE_HOST"));
	let base_url = match Url::parse(&url) {
		Ok(url) => url,
		Err(err) => panic!("Failed to parse Typesense Host: {}", err),
	};

	let client = match Config::new()
		.set_base_url(base_url)
		.add_header("X-Typesense-API-Key", env!("CANISTER_TYPESENSE_KEY"))
	{
		Ok(client) => {
			let client: Client = match client.try_into() {
				Ok(client) => client,
				Err(err) => panic!("Failed to create Typesense Client: {}", err),
			};

			client
		}
		Err(err) => panic!("Failed to create Typesense Client: {}", err),
	};

	match TYPESENSE.set(client) {
		Ok(_) => (),
		Err(_) => panic!("Failed to globalize Typesense Client"),
	}
}

pub fn prisma() -> &'static PrismaClient {
	PRISMA.get().unwrap()
}

pub fn typesense() -> &'static Client {
	match TYPESENSE.get() {
		Some(client) => client,
		None => panic!("Typesense Client not initialized"),
	}
}
