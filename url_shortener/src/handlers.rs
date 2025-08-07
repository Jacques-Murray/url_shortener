use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use nanoid::nanoid;
use sqlx::SqlitePool;
use std::env;

use crate::database::{find_url_by_id, save_url};
use crate::models::{ShortenRequest, ShortenResponse};

pub async fn shorten(
    State(pool): State<SqlitePool>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    let id = nanoid!(7);
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");

    match save_url(&pool, &id, &payload.url).await {
        Ok(_) => {
            let response = ShortenResponse {
                short_url: format!("http://{}/{}", server_address, id),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn redirect(State(pool): State<SqlitePool>, Path(id): Path<String>) -> impl IntoResponse {
    match find_url_by_id(&pool, &id).await {
        Ok(record) => Redirect::to(&record.original_url).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}
