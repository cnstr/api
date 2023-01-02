use crate::db::prisma;
use crate::prisma::package;

use prisma_client_rust::Direction;
use serde_json::{json, to_string_pretty};
use tide::{
	Request, Response, Result,
	StatusCode::{BadRequest, NotFound},
};
use tokio::runtime::Builder;

pub async fn package_lookup(req: Request<()>) -> Result {
	let query = match req.param("package") {
		Ok(query) => query.to_string(),
		Err(err) => {
			println!("Error: {}", err);
			return Ok(Response::builder(BadRequest)
				.body(
					to_string_pretty(&json!({
						"message": "400 Bad Request",
						"error": "Missing URL parameter: \':package\'",
						"date": chrono::Utc::now().to_rfc3339(),
					}))
					.unwrap(),
				)
				.build());
		}
	};

	let packages = Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async move {
			return prisma()
				.await
				.package()
				.find_many(vec![
					package::package::equals(query.to_string()),
					package::is_pruned::equals(false),
				])
				.order_by(package::is_current::order(Direction::Desc))
				.order_by(package::repository_tier::order(Direction::Asc))
				.with(package::repository::fetch())
				.exec()
				.await
				.unwrap();
		});

	if packages.len() == 0 {
		return Ok(Response::builder(NotFound)
			.body(
				to_string_pretty(&json!({
					"message": "404 Not Found",
					"error": "Package not found",
					"date": chrono::Utc::now().to_rfc3339(),
				}))
				.unwrap(),
			)
			.build());
	}

	return Ok(to_string_pretty(&json!({
		"message": "200 Successful",
		"date": chrono::Utc::now().to_rfc3339(),
		"count": packages.len(),
		"data": packages,
	}))
	.unwrap()
	.into());
}
