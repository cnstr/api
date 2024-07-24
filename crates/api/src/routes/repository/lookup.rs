use crate::{
	helpers::{pg_client, responses, row_to_value},
	utility::{api_endpoint, merge_json},
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn lookup(id: Path<String>) -> impl IntoResponse {
	let repository = match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
                        SELECT * FROM repository
                        WHERE
                            visible = true
                            AND id = $1
                        LIMIT 1
                    ",
					&[&id.to_string()],
				)
				.await
			{
				Ok(rows) => rows,
				Err(e) => {
					eprintln!("[db] Failed to query database: {}", e);
					return responses::error(
						StatusCode::INTERNAL_SERVER_ERROR,
						"Failed to query database",
					);
				}
			}
		}
		Err(e) => {
			eprintln!("[db] Failed to query database: {}", e);
			return responses::error(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to query database",
			);
		}
	};

	if repository.len() == 0 {
		return responses::error(StatusCode::NOT_FOUND, "Repository not found");
	}

	let row = &repository[0];
	let id: String = row.get("id");
	let repository = merge_json(
		row_to_value(&row),
		json!({
			"refs": {
				"packages": format!("{}/jailbreak/repository/{}/packages", api_endpoint(), id),
			}
		}),
	);

	responses::data(StatusCode::OK, repository)
}

pub async fn lookup_healthy() -> bool {
	match pg_client().await {
		Ok(pg_client) => {
			match pg_client
				.query(
					"
                        SELECT * FROM repository
                        WHERE
                            visible = true
                            AND id = 'chariz'
                        LIMIT 1
                    ",
					&[],
				)
				.await
			{
				Ok(_) => true,
				Err(_) => false,
			}
		}
		Err(_) => false,
	}
}
