use crate::prisma::package;
use crate::{db::prisma, utility::json_respond};

use prisma_client_rust::Direction;
use serde_json::json;
use tide::{
	Request, Result,
	StatusCode::{BadRequest, NotFound, Ok as OK},
};
use tokio::runtime::Builder;

pub async fn package_lookup(req: Request<()>) -> Result {
	let query = match req.param("package") {
		Ok(query) => query.to_string(),
		Err(err) => {
			println!("Error: {}", err);
			return Ok(json_respond(
				BadRequest,
				json!({
					"message": "400 Bad Request",
					"error": "Missing URL parameter: \':package\'",
					"date": chrono::Utc::now().to_rfc3339(),
				}),
			));
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
		return Ok(json_respond(
			NotFound,
			json!({
				"message": "404 Not Found",
				"error": "Package not found",
				"date": chrono::Utc::now().to_rfc3339(),
			}),
		));
	}

	return Ok(json_respond(
		OK,
		json!({
			"message": "200 Successful",
			"date": chrono::Utc::now().to_rfc3339(),
			"count": packages.len(),
			"data": packages,
		}),
	));
}
