use axum::{
    Router,
    routing::{get, post},
};
use sqlx::SqlitePool;

use crate::handlers::{redirect, shorten};

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", post(shorten))
        .route("/:id", get(redirect))
        .with_state(pool)
}
