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

    // Determine serving mode based on environment variable or debug/release build
    let serve_frontend = std::env::var("SERVE_FRONTEND")
        .map(|v| v == "true" || v == "1")
        .unwrap_or_else(|_| {
            // Default: serve frontend only in release builds
            !cfg!(debug_assertions)
        });

    let frontend_dist = std::path::Path::new("../ui-vite-react/dist");

    if serve_frontend && frontend_dist.exists() {
        // Frontend serving mode: serve built frontend and API
        // No CORS needed since frontend and API are served from same origin
        tracing::info!("🚀 Starting in COMBINED mode (frontend + API)");
        tracing::info!("   Frontend: http://localhost:8080/");
        tracing::info!("   API: http://localhost:8080/api/");
        Router::new()
            .nest("/api", api_routes)
            .nest_service("/assets", ServeDir::new(frontend_dist.join("assets")))
            .route_service("/", ServeFile::new(frontend_dist.join("index.html")))
            .fallback_service(ServeFile::new(frontend_dist.join("index.html")))
    } else {
        // API-only mode: API with CORS for cross-origin requests from Vite dev server
        tracing::info!("🔧 Starting in API-ONLY mode");
        tracing::info!("   API: http://localhost:8080/");
        tracing::info!("   Frontend: Start with 'cd ../ui-vite-react && npm run dev'");
        if !frontend_dist.exists() {
            tracing::info!(
                "   Note: No built frontend found. Run 'npm run build' to enable combined mode."
            );
        }
        api_routes.layer(cors)
    }
}

// Root handler
async fn root() -> &'static str {
    "TPC-C REST API"
}
