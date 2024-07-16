pub struct RuntimeConfig {
	pub meta_name: String,
	pub meta_code: String,
	pub meta_email: String,
	pub meta_copyright: String,

	pub api_endpoint: String,
	pub docs_endpoint: String,
	pub privacy_endpoint: String,
	pub privacy_updated: String,

	pub piracy_url: String,
	pub database_url: String,
	pub typesense_url: String,
	pub vector_url: String,

	pub typesense_api_key: String,
	pub sentry_dsn: String,
}

pub fn load_runtime_config() -> RuntimeConfig {
	RuntimeConfig {
		meta_name: env_or_die("CANISTER_META_NAME"),
		meta_code: env_or_die("CANISTER_META_CODE"),
		meta_email: env_or_die("CANISTER_META_EMAIL"),
		meta_copyright: env_or_die("CANISTER_META_COPYRIGHT"),

		api_endpoint: env_or_die("CANISTER_API_ENDPOINT"),
		docs_endpoint: env_or_die("CANISTER_DOCS_ENDPOINT"),
		privacy_endpoint: env_or_die("CANISTER_PRIVACY_ENDPOINT"),
		privacy_updated: env_or_die("CANISTER_PRIVACY_UPDATED"),

		piracy_url: env_or_die("CANISTER_PIRACY_URL"),
		database_url: env_or_die("CANISTER_DATABASE_URL"),
		typesense_url: env_or_die("CANISTER_TYPESENSE_URL"),
		vector_url: env_or_die("CANISTER_VECTOR_URL"),

		typesense_api_key: env_or_die("CANISTER_TYPESENSE_API_KEY"),
		sentry_dsn: env_or_die("CANISTER_SENTRY_DSN"),
	}
}

fn env_or_die(key: &str) -> String {
	match std::env::var(key) {
		Ok(value) => value,
		Err(_) => {
			eprintln!("FATAL: Missing Environment Variable: {}", key);
			std::process::exit(1);
		}
	}
}
