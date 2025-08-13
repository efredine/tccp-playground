use sqlx::PgPool;
use std::sync::Once;

static INIT: Once = Once::new();

pub async fn setup_test_db() -> PgPool {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    });
    
    // Create test database connection
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/tpcc".to_string());
    
    let pool = PgPool::connect(&database_url).await
        .expect("Failed to connect to test database");
    
    pool
}

pub async fn setup_test_data(pool: &PgPool) {
    // Insert minimal test data for stock level testing
    
    // Insert warehouse
    sqlx::query!(
        "INSERT INTO warehouse1 (w_id, w_name, w_tax, w_ytd) VALUES (1, 'Test Warehouse', 0.10, 300000) ON CONFLICT (w_id) DO NOTHING"
    )
    .execute(pool)
    .await
    .expect("Failed to insert test warehouse");
    
    // Insert district
    sqlx::query!(
        "INSERT INTO district1 (d_id, d_w_id, d_name, d_tax, d_ytd, d_next_o_id) VALUES (1, 1, 'Test District', 0.05, 30000, 25) ON CONFLICT (d_w_id, d_id) DO NOTHING"
    )
    .execute(pool)
    .await
    .expect("Failed to insert test district");
    
    // Insert test items
    for i in 1..=5 {
        sqlx::query!(
            "INSERT INTO item1 (i_id, i_im_id, i_name, i_price, i_data) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (i_id) DO NOTHING",
            i,
            100 + i,
            format!("Test Item {}", i),
            bigdecimal::BigDecimal::from(10 + i),
            format!("Test data {}", i)
        )
        .execute(pool)
        .await
        .expect("Failed to insert test item");
    }
    
    // Insert test stock (some below threshold, some above)
    for i in 1..=5 {
        let quantity = if i <= 2 { 5 } else { 25 }; // Items 1,2 are below threshold 10
        sqlx::query!(
            "INSERT INTO stock1 (s_i_id, s_w_id, s_quantity, s_ytd, s_order_cnt, s_remote_cnt, s_data) 
             VALUES ($1, 1, $2, 0, 0, 0, 'test') ON CONFLICT (s_w_id, s_i_id) DO NOTHING",
            i,
            quantity
        )
        .execute(pool)
        .await
        .expect("Failed to insert test stock");
    }
    
    // Insert test orders and order lines for recent orders (5-24, so that next_o_id=25)
    for o_id in 5..25 {
        // Insert order
        sqlx::query!(
            "INSERT INTO orders1 (o_id, o_d_id, o_w_id, o_c_id, o_entry_d, o_ol_cnt, o_all_local) 
             VALUES ($1, 1, 1, 1, NOW(), 2, 1) ON CONFLICT (o_w_id, o_d_id, o_id) DO NOTHING",
            o_id
        )
        .execute(pool)
        .await
        .expect("Failed to insert test order");
        
        // Insert order lines (using items 1 and 2 which are below threshold)
        for ol_number in 1i16..=2i16 {
            let item_id = ((o_id + ol_number as i32 - 1) % 5) + 1; // Cycle through items 1-5
            sqlx::query!(
                "INSERT INTO order_line1 (ol_o_id, ol_d_id, ol_w_id, ol_number, ol_i_id, ol_supply_w_id, ol_quantity, ol_amount, ol_dist_info) 
                 VALUES ($1, 1, 1, $2, $3, 1, 5, 50.0, 'test_dist') ON CONFLICT (ol_w_id, ol_d_id, ol_o_id, ol_number) DO NOTHING",
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