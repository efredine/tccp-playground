use dotenvy::dotenv;
use rust_axum_rest_api::create_app;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // initialize tracing for logging
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    info!("Connected to the database!");

    let app = create_app(pool).await;

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    info!("Server is running on http://127.0.0.1:8080");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
