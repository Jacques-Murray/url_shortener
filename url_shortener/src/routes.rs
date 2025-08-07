use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;

use crate::handlers::{redirect, shorten};

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", post(shorten))
        // This line is now corrected to use {id}
        .route("/:id", get(redirect))
        .with_state(pool)
}
