use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

pub mod handlers;
pub mod models;

use handlers::*;

// Factory function to create the app router
pub async fn create_app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/warehouses", get(get_warehouses))
        .route("/stock-level", get(stock_level))
        .with_state(pool)
}

// Root handler
async fn root() -> &'static str {
    "TPC-C REST API"
}
