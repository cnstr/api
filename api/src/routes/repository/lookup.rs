use crate::db::prisma;
use crate::prisma::repository;

use serde_json::{json, to_string_pretty};
use tide::{
	Request, Response, Result,
	StatusCode::{BadRequest, NotFound},
};
use tokio::runtime::Builder;

pub async fn repository_lookup(req: Request<()>) -> Result {
	let query = match req.param("repository") {
		Ok(query) => query.to_string(),
		Err(err) => {
			println!("Error: {}", err);
			return Ok(Response::builder(BadRequest)
				.body(
					to_string_pretty(&json!({
						"message": "400 Bad Request",
						"error": "Missing URL parameter: \':repository\'",
						"date": chrono::Utc::now().to_rfc3339(),
					}))
					.unwrap(),
				)
				.build());
		}
	};

	let repository = Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async move {
			return prisma()
				.await
				.repository()
				.find_first(vec![
					repository::slug::equals(query.to_string()),
					repository::is_pruned::equals(false),
				])
				.with(repository::origin::fetch())
				.exec()
				.await
				.unwrap();
		});

	match repository {
		Some(repository) => {
			return Ok(to_string_pretty(&json!({
				"message": "200 Successful",
				"date": chrono::Utc::now().to_rfc3339(),
				"data": repository,
			}))
			.unwrap()
			.into());
		}
		None => {
			return Ok(Response::builder(NotFound)
				.body(
					to_string_pretty(&json!({
						"message": "404 Not Found",
						"error": "Repository not found",
						"date": chrono::Utc::now().to_rfc3339(),
					}))
					.unwrap(),
				)
				.build());
		}
	}
}
