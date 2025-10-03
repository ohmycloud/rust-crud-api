use std::sync::Arc;

use crate::handler::hello_world;
use axum::{Router, routing::get};
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

mod handler;

pub struct AppState {
    pub db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    // try to load the .env file
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");
    let app_state = Arc::new(AppState {
        db_pool: pool.clone(),
    });

    let app = Router::new()
        .route("/api", get(hello_world))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
