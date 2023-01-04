pub use crate::prisma;

use self::prisma::PrismaClient;
use elasticsearch::{http::transport::Transport, Elasticsearch};
use once_cell::sync::OnceCell;

static PRISMA: OnceCell<PrismaClient> = OnceCell::new();
static ELASTIC: OnceCell<Elasticsearch> = OnceCell::new();

pub async fn create_prisma() {
	let client = PrismaClient::_builder()
		.with_url("postgresql://cnstr:pg@localhost:5432/cnstr".to_string())
		.build()
		.await;

	match client {
		Ok(client) => PRISMA.set(client).unwrap(),
		Err(err) => panic!("Failed to connect to database: {}", err),
	}
}

pub async fn create_elastic() {
	let transport = Transport::single_node("http://localhost:9200").unwrap();
	let client = Elasticsearch::new(transport);
	ELASTIC.set(client).unwrap()
}

pub fn prisma() -> &'static PrismaClient {
	PRISMA.get().unwrap()
}

pub fn elastic() -> &'static Elasticsearch {
	ELASTIC.get().unwrap()
}
