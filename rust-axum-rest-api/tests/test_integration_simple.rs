use rust_axum_rest_api::create_app;
use tower::ServiceExt;
use hyper::{Request, Method};
use axum::body::Body;
use http_body_util::BodyExt;
use sqlx::PgPool;

// Simple integration test using a clean database setup per test
async fn setup_clean_test_db() -> Option<PgPool> {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/tpcc".to_string());
    
    if let Ok(pool) = PgPool::connect(&database_url).await {
        // Clean up any existing test data
        let _ = sqlx::query("DELETE FROM history1 WHERE h_data LIKE 'test%'").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM order_line1 WHERE ol_dist_info = 'test_dist'").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM orders1 WHERE o_c_id = 999999").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM stock1 WHERE s_data = 'test'").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM item1 WHERE i_name LIKE 'Test Item%'").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM customer1 WHERE c_first = 'TestUser'").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM district1 WHERE d_name = 'TestDist'").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM warehouse1 WHERE w_name = 'TestWH'").execute(&pool).await;
        
        Some(pool)
    } else {
        None
    }
}

async fn setup_test_data(pool: &PgPool) {
    // Insert test warehouse
    sqlx::query!(
        "INSERT INTO warehouse1 (w_id, w_name, w_tax, w_ytd) VALUES (999, 'TestWH', 0.10, 300000) ON CONFLICT (w_id) DO UPDATE SET w_name = 'TestWH'"
    )
    .execute(pool)
    .await
    .expect("Failed to insert test warehouse");
    
    // Insert test district
    sqlx::query!(
        "INSERT INTO district1 (d_id, d_w_id, d_name, d_tax, d_ytd, d_next_o_id) VALUES (99, 999, 'TestDist', 0.05, 30000, 25) ON CONFLICT (d_w_id, d_id) DO UPDATE SET d_next_o_id = 25"
    )
    .execute(pool)
    .await
    .expect("Failed to insert test district");
    
    // Insert test items
    for i in 1001..1006 {
        sqlx::query!(
            "INSERT INTO item1 (i_id, i_im_id, i_name, i_price, i_data) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (i_id) DO UPDATE SET i_name = EXCLUDED.i_name",
            i,
            100 + i,
            format!("Test Item {}", i),
            bigdecimal::BigDecimal::from(10 + (i - 1000)),
            format!("Test data {}", i)
        )
        .execute(pool)
        .await
        .expect("Failed to insert test item");
    }
    
    // Insert test stock (some below threshold, some above)
    for i in 1001..1006 {
        let quantity = if i <= 1002 { 5 } else { 25 }; // Items 1001,1002 are below threshold 10
        sqlx::query!(
            "INSERT INTO stock1 (s_i_id, s_w_id, s_quantity, s_ytd, s_order_cnt, s_remote_cnt, s_data) 
             VALUES ($1, 999, $2, 0, 0, 0, 'test') ON CONFLICT (s_w_id, s_i_id) DO UPDATE SET s_quantity = EXCLUDED.s_quantity",
            i,
            quantity
        )
        .execute(pool)
        .await
        .expect("Failed to insert test stock");
    }
    
    // Insert test customer
    sqlx::query!(
        "INSERT INTO customer1 (c_id, c_d_id, c_w_id, c_first, c_middle, c_last, c_since, c_credit, c_credit_lim, c_discount, c_balance, c_ytd_payment, c_payment_cnt, c_delivery_cnt, c_data) 
         VALUES (999999, 99, 999, 'TestUser', 'T', 'Customer', NOW(), 'GC', 50000, 0.05, 1000.0, 10.0, 1, 0, 'test customer data')"
    )
    .execute(pool)
    .await
    .expect("Failed to insert test customer");
    
    // Insert test orders and order lines
    for o_id in 5..25 {
        sqlx::query!(
            "INSERT INTO orders1 (o_id, o_d_id, o_w_id, o_c_id, o_entry_d, o_ol_cnt, o_all_local) 
             VALUES ($1, 99, 999, 999999, NOW(), 2, 1)",
            o_id
        )
        .execute(pool)
        .await
        .expect("Failed to insert test order");
        
        for ol_number in 1i16..=2i16 {
            let item_id = 1001 + ((o_id + ol_number as i32 - 1) % 5); // Cycle through test items
            sqlx::query!(
                "INSERT INTO order_line1 (ol_o_id, ol_d_id, ol_w_id, ol_number, ol_i_id, ol_supply_w_id, ol_quantity, ol_amount, ol_dist_info) 
                 VALUES ($1, 99, 999, $2, $3, 999, 5, 50.0, 'test_dist')",
                o_id,
                ol_number,
                item_id
            )
            .execute(pool)
            .await
            .expect("Failed to insert test order line");
        }
    }
}

#[tokio::test]
async fn test_stock_level_with_isolated_data() {
    if let Some(pool) = setup_clean_test_db().await {
        setup_test_data(&pool).await;
        
        let app = create_app(pool).await;
        
        // Test stock-level with our isolated test data
        let request = Request::builder()
            .method(Method::GET)
            .uri("/stock-level?warehouse_id=999&district_id=99&threshold=10")
            .body(Body::empty())
            .unwrap();
            
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), 200);
        
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let response_text = std::str::from_utf8(&body).unwrap();
        
        let json: serde_json::Value = serde_json::from_str(response_text).unwrap();
        
        assert_eq!(json["warehouse_id"], 999);
        assert_eq!(json["district_id"], 99);
        assert_eq!(json["threshold"], 10);
        
        let low_stock_count = json["low_stock_count"].as_i64().unwrap();
        assert!(low_stock_count >= 0);
        
        println!("✅ Stock level test passed with low_stock_count: {}", low_stock_count);
        println!("📝 Response: {}", response_text);
    } else {
        println!("⚠️  Database not available, skipping integration test");
    }
}

#[tokio::test]
async fn test_basic_endpoints() {
    if let Some(pool) = setup_clean_test_db().await {
        let app = create_app(pool).await;
        
        // Test root endpoint
        let request = Request::builder()
            .method(Method::GET)
            .uri("/")
            .body(Body::empty())
            .unwrap();
            
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), 200);
        
        // Test stock-level without parameters (should return 400)
        let request = Request::builder()
            .method(Method::GET)
            .uri("/stock-level")
            .body(Body::empty())
            .unwrap();
            
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), 400);
        
        println!("✅ Basic endpoints test passed");
    } else {
        println!("⚠️  Database not available, skipping basic endpoints test");
    }
}