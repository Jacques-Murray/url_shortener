mod database;
mod handlers;
mod models;
mod routes;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = database::create_db_pool()
        .await
        .expect("Failed to create database pool");

    let app = routes::create_router(pool);

    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set");
    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();
    println!("Listening on {}", server_address);

    axum::serve(listener, app).await.unwrap();
}
