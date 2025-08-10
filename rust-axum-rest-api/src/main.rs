use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing::{Level, info};

mod handlers;
mod models;

use handlers::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // initialize tracing for logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database!");

    let app = Router::new()
        .route("/", get(root))
        .route("/warehouses", get(get_warehouses))
        .with_state(pool);

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();
    info!("Server is running on http://127.0.0.1:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// handler for GET /
async fn root() -> &'static str {
    "Hello, world!"
}
