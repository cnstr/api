use tide::{Request, Result};

pub async fn openapi_yaml(_req: Request<()>) -> Result {
	return Ok(("").into());
}

pub async fn openapi_json(_req: Request<()>) -> Result {
	return Ok(("").into());
}
