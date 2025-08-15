use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{Pool, Postgres};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

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

    // API routes without CORS initially
    let api_routes = Router::new()
        .route("/", get(root))
        .route("/warehouses", get(get_warehouses))
        .route("/districts", get(get_districts))
        .route("/customers", get(search_customers))
        .route("/items", get(search_items))
        .route("/stock", get(get_stock_info))
        .route("/stock-level", get(stock_level))
        .route("/order-status", get(order_status))
        .route("/orders", get(list_orders))
        .route("/new-order", post(new_order))
        .route("/payment", post(payment))
        .route("/delivery", post(delivery))
        .with_state(pool);

    // Check if we have a built frontend to serve
    let frontend_dist = std::path::Path::new("../ui-vite-react/dist");
    
    if frontend_dist.exists() {
        // Production mode: serve built frontend and API
        // No CORS needed since frontend and API are served from same origin
        Router::new()
            .nest("/api", api_routes)
            .fallback_service(ServeDir::new(frontend_dist)
                .not_found_service(ServeFile::new(frontend_dist.join("index.html"))))
    } else {
        // Development mode: API only with CORS for cross-origin requests from Vite dev server
        api_routes.layer(cors)
    }
}

// Root handler
async fn root() -> &'static str {
    "TPC-C REST API"
}
