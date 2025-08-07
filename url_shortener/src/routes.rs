pub use crate::AppState;
use crate::handlers::{redirect, shorten};
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", post(shorten))
        .route("/{id}", get(redirect))
        .with_state(app_state)
}
