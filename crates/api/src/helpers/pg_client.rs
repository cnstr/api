use crate::utility::load_runtime_config;
use anyhow::Result;
use deadpool_postgres::{
	tokio_postgres::{types::Type, Row},
	Client, Config as PgConfig, ManagerConfig, Pool, RecyclingMethod, Runtime,
};
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use serde_json::{Map, Value};
use std::sync::OnceLock;

static DB_POOL: OnceLock<Pool> = OnceLock::new();

pub async fn create_db() -> Result<()> {
	if DB_POOL.get().is_some() {
		return Ok(());
	}

	let config = load_runtime_config();

	let mut pg = PgConfig::new();
	pg.url = Some(config.database_url);
	pg.manager = Some(ManagerConfig {
		recycling_method: RecyclingMethod::Fast,
	});

	let ssl_builder = match SslConnector::builder(SslMethod::tls()) {
		Ok(builder) => builder,
		Err(e) => {
			eprintln!("[db] Failed to create SSL builder: {}", e);
			return Err(e.into());
		}
	};

	let connector = MakeTlsConnector::new(ssl_builder.build());
	let pool = match pg.create_pool(Some(Runtime::Tokio1), connector) {
		Ok(pool) => pool,
		Err(e) => {
			eprintln!("[db] Failed to create pool: {}", e);
			return Err(e.into());
		}
	};

	println!("[db] Opened a pool with the database");
	match DB_POOL.set(pool) {
		Ok(_) => Ok(()),
		Err(_) => {
			eprintln!("[db] Failed to set the pool");
			Err(anyhow::anyhow!("Failed to set the pool"))
		}
	}
}

pub async fn pg_client() -> Result<Client> {
	let pool = match DB_POOL.get() {
		Some(pool) => pool,
		None => {
			eprintln!("[db] Pool not initialized");
			return Err(anyhow::anyhow!("Pool not initialized"));
		}
	};

	match pool.get().await {
		Ok(client) => Ok(client),
		Err(e) => {
			eprintln!("[db] Failed to get a client: {}", e);
			Err(e.into())
		}
	}
}

pub fn row_to_value(row: &Row) -> Value {
	let mut obj = Map::new();

	for (i, column) in row.columns().iter().enumerate() {
		let column_name = column.name();
		let column_value = match *column.type_() {
			Type::BOOL => row
				.get::<usize, Option<bool>>(i)
				.map_or(Value::Null, Into::into),
			Type::INT2 => row
				.get::<usize, Option<i16>>(i)
				.map_or(Value::Null, Into::into),
			Type::INT4 => row
				.get::<usize, Option<i32>>(i)
				.map_or(Value::Null, Into::into),
			Type::INT8 => row
				.get::<usize, Option<i64>>(i)
				.map_or(Value::Null, Into::into),
			Type::TEXT | Type::VARCHAR => row
				.get::<usize, Option<String>>(i)
				.map_or(Value::Null, Into::into),
			_ => Value::Null,
		};

		obj.insert(column_name.to_string(), column_value);
	}

	Value::Object(obj)
}
