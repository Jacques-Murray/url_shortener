use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}

#[derive(sqlx::FromRow)]
pub struct UrlRecord {
    pub id: String,
    pub original_url: String,
}
