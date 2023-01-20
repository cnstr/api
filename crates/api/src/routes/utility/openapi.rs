use tide::{Request, Response, Result};

pub async fn openapi_yaml(_req: Request<()>) -> Result {
	let body = env!("CANISTER_OPENAPI_YAML").replace("\\n", "\n");

	let res = Response::builder(200)
		.header("Content-Type", "text/yaml")
		.body(body)
		.build();

	return Ok(res);
}

pub async fn openapi_json(_req: Request<()>) -> Result {
	let body = env!("CANISTER_OPENAPI_JSON");

	let res = Response::builder(200)
		.header("Content-Type", "application/json")
		.body(body)
		.build();

	return Ok(res);
}
