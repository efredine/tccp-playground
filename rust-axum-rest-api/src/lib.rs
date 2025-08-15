use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};
use tower_http::cors::{CorsLayer, Any};

pub mod handlers;
pub mod models;

use handlers::*;

// Factory function to create the app router
pub async fn create_app(pool: Pool<Postgres>) -> Router {
    // Configure CORS for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(root))
        .route("/warehouses", get(get_warehouses))
        .route("/districts", get(get_districts))
        .route("/customers", get(search_customers))
        .route("/items", get(search_items))
        .route("/stock", get(get_stock_info))
        .route("/stock-level", get(stock_level))
        .route("/order-status", get(order_status))
        .route("/new-order", post(new_order))
        .route("/payment", post(payment))
        .route("/delivery", post(delivery))
        .layer(cors)
        .with_state(pool)
}

// Root handler
async fn root() -> &'static str {
    "TPC-C REST API"
}
