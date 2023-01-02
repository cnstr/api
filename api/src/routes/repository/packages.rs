use crate::prisma::repository;
use crate::{db::prisma, prisma::package};

use serde_json::{json, to_string_pretty};
use tide::{
	Request, Response, Result,
	StatusCode::{BadRequest, NotFound},
};
use tokio::runtime::Builder;

pub async fn repository_packages(req: Request<()>) -> Result {
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

	let request = Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async move {
			let repository = prisma()
				.await
				.repository()
				.find_first(vec![
					repository::slug::equals(query.to_string()),
					repository::is_pruned::equals(false),
				])
				.exec()
				.await
				.unwrap();

			return match repository {
				Some(repository) => Ok(prisma()
					.await
					.package()
					.find_many(vec![package::repository_slug::equals(repository.slug)])
					.exec()
					.await
					.unwrap()),
				None => Err(Ok(Response::builder(NotFound)
					.body(
						to_string_pretty(&json!({
							"message": "404 Not Found",
							"error": "Repository not found",
							"date": chrono::Utc::now().to_rfc3339(),
						}))
						.unwrap(),
					)
					.build())),
			};
		});

	let packages = match request {
		Ok(packages) => packages,
		Err(response) => return response,
	};

	return Ok(to_string_pretty(&json!({
		"message": "200 Successful",
		"date": chrono::Utc::now().to_rfc3339(),
		"count": packages.len(),
		"data": packages,
	}))
	.unwrap()
	.into());
}
