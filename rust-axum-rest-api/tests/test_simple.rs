// Simple integration test without complicated test server setup
use axum::body::Body;
use hyper::{Method, Request};
use rust_axum_rest_api::create_app;
use sqlx::PgPool;
use tower::ServiceExt;

#[tokio::test]
async fn test_app_creation() {
    // Test that we can create the app (basic smoke test)
    let database_url = "postgres://postgres:password@localhost:5432/tpcc";

    // Try to connect, but don't panic if database isn't available
    if let Ok(pool) = PgPool::connect(database_url).await {
        let app = create_app(pool).await;

        // Test root endpoint
        let request = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), 200);
    } else {
        println!("Database not available, skipping test");
    }
}

#[tokio::test]
async fn test_stock_level_missing_params() {
    let database_url = "postgres://postgres:password@localhost:5432/tpcc";

    if let Ok(pool) = PgPool::connect(database_url).await {
        let app = create_app(pool).await;

        // Test stock-level without parameters (should fail)
        let request = Request::builder()
            .method(Method::GET)
            .uri("/stock-level")
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        // Should return 400 due to missing query parameters
        assert_eq!(response.status(), 400);
    } else {
        println!("Database not available, skipping test");
    }
}
