use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use sqlx::SqlitePool;

use crate::database::find_url_by_id;

pub async fn redirect(State(pool): State<SqlitePool>, Path(id): Path<String>) -> impl IntoResponse {
    match find_url_by_id(&pool, &id).await {
        Ok(record) => Redirect::to(&record.original_url).into_response(),
        Err(e) => {
            eprintln!("Error finding URL by id '{}': {:?}", id, e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}
