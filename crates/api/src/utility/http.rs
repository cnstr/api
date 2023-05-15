#[derive(Debug)]
pub struct Brand {
	pub name: String,
	pub r#type: String,
	pub version: String,
}

fn parse_brand(input: &str) -> Option<Brand> {
	let mut name = String::new();
	let mut r#type = String::new();
	let mut version = String::new();

	for item in input.split(';') {
		let kv_pair: Vec<&str> = item.split('=').collect();

		match kv_pair.len() {
			// Brand name doesn't follow the key=value format
			1 => name = kv_pair[0].to_string(),
			2 => match kv_pair[0] {
				"t" => r#type = kv_pair[1].to_string(),
				"v" => version = kv_pair[1].to_string(),
				_ => (),
			},
			_ => (),
		}
	}

	if name.is_empty() {
		return None;
	}

	if r#type.is_empty() {
		r#type = "unknown".to_string();
	}

	if version.is_empty() {
		version = "unknown".to_string();
	}

	Some(Brand {
		name,
		r#type,
		version,
	})
}

pub fn parse_user_agent(input: &str) -> Vec<Brand> {
	let mut brands = Vec::new();

	for item in input.split(',') {
		if let Some(brand) = parse_brand(item) {
			brands.push(brand);
		}
	}

	brands
}
