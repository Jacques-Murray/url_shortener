use crate::AppState;
use crate::database::{find_url_by_id, save_url};
use crate::models::{ErrorResponse, ShortenRequest, ShortenResponse};
use axum::Json;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

pub async fn redirect(
    State(app_state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let pool = &app_state.pool;
    match find_url_by_id(pool, &id).await {
        Ok(record) => Redirect::to(&record.original_url).into_response(),
        Err(e) => {
            eprintln!("Error finding URL by id '{}': {:?}", id, e);
            StatusCode::NOT_FOUND.into_response()
        }
    }
}

pub async fn shorten(
    State(app_state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> axum::response::Response {
    use nanoid::nanoid;
    let id = nanoid!(7);
    let pool = &app_state.pool;
    let config = &app_state.config;

    match save_url(pool, &id, &payload.url).await {
        Ok(_) => {
            let response = ShortenResponse {
                short_url: format!("{}://{}/{}", config.protocol, config.server_address, id),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => {
            eprintln!("Failed to save URL: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to save URL".to_string(),
                }),
            )
                .into_response()
        }
    }
}
