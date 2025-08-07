mod database;
mod handlers;
mod models;
mod routes;

use dotenvy::dotenv;
use std::env;
// Configuration struct for environment-based settings
// AppState struct to hold both SqlitePool and Config
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub config: Config,
}
#[derive(Clone)]
pub struct Config {
    pub server_address: String,
    pub protocol: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = database::create_db_pool()
        .await
        .expect("Failed to create database pool");

    // Load configuration from environment
    let config = Config {
        server_address: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "localhost:3000".to_string()),
        protocol: env::var("SERVER_PROTOCOL").unwrap_or_else(|_| "http".to_string()),
    };

    let app_state = AppState {
        pool,
        config: config.clone(),
    };

    let app = routes::create_router(app_state.clone());

    let listener = tokio::net::TcpListener::bind(&config.server_address)
        .await
        .unwrap();
    println!("Listening on {}", config.server_address);

    axum::serve(listener, app).await.unwrap();
}
