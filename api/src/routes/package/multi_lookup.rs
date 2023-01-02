use crate::db::prisma;
use crate::prisma::package;

use prisma_client_rust::Direction;
use serde_json::{json, to_string_pretty};
use tide::{
	prelude::Deserialize,
	Request, Response, Result,
	StatusCode::{BadRequest, NotFound, UnprocessableEntity},
};
use tokio::runtime::Builder;

#[derive(Deserialize)]
struct Query {
	ids: Option<String>,
}

pub async fn package_multi_lookup(req: Request<()>) -> Result {
	let ids = match req.query::<Query>() {
		Ok(query) => {
			let ids = match query.ids {
				Some(ids) => {
					let ids: Vec<String> = ids.split(',').map(|id| id.to_string()).collect();
					ids
				}
				None => {
					return Ok(Response::builder(BadRequest)
						.body(
							to_string_pretty(&json!({
								"message": "400 Bad Request",
								"error": "Missing query parameter: \'ids\'",
								"date": chrono::Utc::now().to_rfc3339(),
							}))
							.unwrap(),
						)
						.build());
				}
			};

			ids
		}

		Err(err) => {
			println!("Error: {}", err);
			return Ok(Response::builder(UnprocessableEntity)
				.body(
					to_string_pretty(&json!({
						"message": "422 Unprocessable Entity",
						"error": "Malformed query parameters",
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
					package::package::in_vec(ids),
					package::is_current::equals(true),
					package::is_pruned::equals(false),
				])
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
					"error": "Packages not found",
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
