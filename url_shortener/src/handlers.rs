use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use sqlx::SqlitePool;

use crate::database::{find_url_by_id, save_url};

pub async fn redirect(State(pool): State<SqlitePool>, Path(id): Path<String>) -> impl IntoResponse {
    match find_url_by_id(&pool, &id).await {
        Ok(record) => Redirect::to(&record.original_url).into_response(),
        Err(e) => {
            eprintln!("Error finding URL by id '{}': {:?}", id, e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}
use crate::models::{ShortenRequest, ShortenResponse};
use axum::Json;

pub async fn shorten(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    use nanoid::nanoid;
    let id = nanoid!(7);
    let server_address =
        std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "localhost:3000".to_string());
    let protocol = std::env::var("SERVER_PROTOCOL").unwrap_or_else(|_| "http".to_string());

    match save_url(&pool, &id, &payload.url).await {
        Ok(_) => {
            let response = ShortenResponse {
                short_url: format!("{}://{}/{}", protocol, server_address, id),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            eprintln!("Failed to save URL: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ShortenResponse {
                    short_url: "".to_string(),
                }),
            )
        }
    }
}
