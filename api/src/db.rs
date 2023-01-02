pub use crate::prisma;

use self::prisma::PrismaClient;
use elasticsearch::{http::transport::Transport, Elasticsearch};

pub async fn prisma() -> PrismaClient {
	return PrismaClient::_builder()
		.with_url("postgresql://cnstr:pg@localhost:5432/cnstr".to_string())
		.build()
		.await
		.unwrap();
}

pub async fn elastic() -> elasticsearch::Elasticsearch {
	let transport = Transport::single_node("http://localhost:9200").unwrap();
	return Elasticsearch::new(transport);
}
