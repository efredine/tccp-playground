use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

// Request Query Parameters for order listing
#[derive(Deserialize)]
pub struct OrdersQuery {
    // Filtering criteria
    pub warehouse_id: Option<i16>,
    pub district_id: Option<i16>,
    pub customer_id: Option<i32>,
    pub order_id: Option<i32>,
    
    // Date range filtering
    pub from_date: Option<String>, // ISO date string
    pub to_date: Option<String>,   // ISO date string
    
    // Pagination
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    
    // Sorting
    pub sort_by: Option<String>,   // "order_id", "entry_date", "customer_last", etc.
    pub sort_dir: Option<String>,  // "asc" or "desc"
}

// Response structures for order listing
#[derive(Serialize)]
pub struct OrdersListResponse {
    pub orders: Vec<OrderSummary>,
    pub total_count: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[derive(Serialize)]
pub struct OrderSummary {
    pub o_id: i32,
    pub o_w_id: i16,
    pub o_d_id: i16,
    pub o_c_id: Option<i32>,
    pub o_entry_d: Option<NaiveDateTime>,
    pub o_carrier_id: Option<i16>,
    pub o_ol_cnt: Option<i16>,
    pub o_all_local: Option<i16>,
    
    // Customer info for display
    pub customer_first: Option<String>,
    pub customer_middle: Option<String>,
    pub customer_last: Option<String>,
    
    // Order total (calculated from order lines)
    pub total_amount: Option<BigDecimal>,
    
    // Status indicators
    pub is_delivered: bool,
    pub line_count: i64,
}

// Handler function for listing orders
pub async fn list_orders(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<OrdersQuery>,
) -> Result<Json<OrdersListResponse>, StatusCode> {
    // Set defaults for pagination
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100); // Cap at 100 per page
    let offset = (page - 1) * per_page;
    
    // Set defaults for sorting
    let sort_by = params.sort_by.as_deref().unwrap_or("o_entry_d");
    let _sort_dir = params.sort_dir.as_deref().unwrap_or("desc");
    
    // For this initial version, we'll do basic pagination without dynamic filters
    // TODO: Add dynamic filtering in a future enhancement
    let _sort_column = match sort_by {
        "order_id" => "o.o_id",
        "entry_date" => "o.o_entry_d",
        "customer_last" => "c.c_last",
        "warehouse_id" => "o.o_w_id",
        "district_id" => "o.o_d_id",
        "carrier_id" => "o.o_carrier_id",
        _ => "o.o_entry_d", // default fallback
    };
    
    // Get the total count
    let total_count = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM orders1"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error counting orders: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .unwrap_or(0);
    
    // For this initial implementation, let's do a simple query without dynamic filtering
    // We can enhance this later with proper parameter binding
    let orders_rows = sqlx::query!(
        r#"
        WITH order_totals AS (
            SELECT 
                ol_w_id, 
                ol_d_id, 
                ol_o_id,
                SUM(ol_amount) as total_amount,
                COUNT(*) as line_count
            FROM order_line1 
            GROUP BY ol_w_id, ol_d_id, ol_o_id
        )
        SELECT 
            o.o_id,
            o.o_w_id,
            o.o_d_id, 
            o.o_c_id,
            o.o_entry_d,
            o.o_carrier_id,
            o.o_ol_cnt,
            o.o_all_local,
            c.c_first,
            c.c_middle,
            c.c_last,
            COALESCE(ot.total_amount, 0) as total_amount,
            CASE 
                WHEN o.o_carrier_id IS NOT NULL THEN true 
                ELSE false 
            END as is_delivered,
            COALESCE(ot.line_count, 0) as line_count
        FROM orders1 o
        LEFT JOIN customer1 c ON o.o_w_id = c.c_w_id AND o.o_d_id = c.c_d_id AND o.o_c_id = c.c_id
        LEFT JOIN order_totals ot ON o.o_w_id = ot.ol_w_id AND o.o_d_id = ot.ol_d_id AND o.o_id = ot.ol_o_id
        ORDER BY o.o_entry_d DESC, o.o_w_id, o.o_d_id, o.o_id
        LIMIT $1 OFFSET $2
        "#,
        per_page as i64,
        offset as i64
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching orders: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    // Convert to OrderSummary structs
    let orders: Vec<OrderSummary> = orders_rows
        .into_iter()
        .map(|row| OrderSummary {
            o_id: row.o_id,
            o_w_id: row.o_w_id,
            o_d_id: row.o_d_id,
            o_c_id: row.o_c_id,
            o_entry_d: row.o_entry_d,
            o_carrier_id: row.o_carrier_id,
            o_ol_cnt: row.o_ol_cnt,
            o_all_local: row.o_all_local,
            customer_first: row.c_first,
            customer_middle: row.c_middle,
            customer_last: row.c_last,
            total_amount: row.total_amount,
            is_delivered: row.is_delivered.unwrap_or(false),
            line_count: row.line_count.unwrap_or(0),
        })
        .collect();
    
    let total_pages = ((total_count as f64) / (per_page as f64)).ceil() as u32;
    
    Ok(Json(OrdersListResponse {
        orders,
        total_count,
        page,
        per_page,
        total_pages,
    }))
}