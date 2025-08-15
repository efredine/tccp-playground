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
    pub sort_by: Option<String>, // "order_id", "entry_date", "customer_last", etc.
    pub sort_dir: Option<String>, // "asc" or "desc"
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
    let sort_by = params.sort_by.as_deref().unwrap_or("entry_date");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("desc");

    // Validate and map sort column for security
    let sort_column = match sort_by {
        "order_id" => "o.o_id",
        "entry_date" => "o.o_entry_d",
        "customer_last" => "c.c_last",
        "warehouse_id" => "o.o_w_id",
        "district_id" => "o.o_d_id",
        "carrier_id" => "o.o_carrier_id",
        _ => "o.o_entry_d", // default fallback for security
    };

    // Validate sort direction for security
    let sort_direction = match sort_dir.to_lowercase().as_str() {
        "asc" => "ASC",
        "desc" => "DESC",
        _ => "DESC", // default fallback for security
    };

    // Get the total count
    let total_count = sqlx::query_scalar!("SELECT COUNT(*) FROM orders1")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error counting orders: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .unwrap_or(0);

    // Build dynamic query with user-specified sorting
    let query = format!(
        r#"
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
            CASE 
                WHEN o.o_carrier_id IS NOT NULL THEN true 
                ELSE false 
            END as is_delivered
        FROM orders1 o
        LEFT JOIN customer1 c ON o.o_w_id = c.c_w_id AND o.o_d_id = c.c_d_id AND o.o_c_id = c.c_id
        ORDER BY {} {}, o.o_w_id, o.o_d_id, o.o_id
        LIMIT $1 OFFSET $2
        "#,
        sort_column, sort_direction
    );

    let orders_rows = sqlx::query_as::<
        _,
        (
            i32,                           // o_id
            i16,                           // o_w_id
            i16,                           // o_d_id
            Option<i32>,                   // o_c_id
            Option<chrono::NaiveDateTime>, // o_entry_d
            Option<i16>,                   // o_carrier_id
            Option<i16>,                   // o_ol_cnt
            Option<i16>,                   // o_all_local
            Option<String>,                // c_first
            Option<String>,                // c_middle
            Option<String>,                // c_last
            Option<bool>,                  // is_delivered
        ),
    >(&query)
    .bind(per_page as i64)
    .bind(offset as i64)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching orders: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // If we have orders, calculate totals efficiently for only these orders
    let mut orders: Vec<OrderSummary> = orders_rows
        .into_iter()
        .map(|row| OrderSummary {
            o_id: row.0,                           // o_id
            o_w_id: row.1,                         // o_w_id
            o_d_id: row.2,                         // o_d_id
            o_c_id: row.3,                         // o_c_id
            o_entry_d: row.4,                      // o_entry_d
            o_carrier_id: row.5,                   // o_carrier_id
            o_ol_cnt: row.6,                       // o_ol_cnt
            o_all_local: row.7,                    // o_all_local
            customer_first: row.8,                 // c_first
            customer_middle: row.9,                // c_middle
            customer_last: row.10,                 // c_last
            total_amount: None,                    // Will be filled in below
            is_delivered: row.11.unwrap_or(false), // is_delivered
            line_count: 0,                         // Will be filled in below
        })
        .collect();

    // Calculate totals for only the orders we fetched (much more efficient)
    if !orders.is_empty() {
        // Build the condition for only the orders we need
        let order_conditions: Vec<String> = orders
            .iter()
            .map(|o| {
                format!(
                    "(ol_w_id = {} AND ol_d_id = {} AND ol_o_id = {})",
                    o.o_w_id, o.o_d_id, o.o_id
                )
            })
            .collect();

        if !order_conditions.is_empty() {
            let totals_query = format!(
                r#"
                SELECT 
                    ol_w_id, 
                    ol_d_id, 
                    ol_o_id,
                    SUM(ol_amount) as total_amount,
                    COUNT(*) as line_count
                FROM order_line1 
                WHERE {}
                GROUP BY ol_w_id, ol_d_id, ol_o_id
                "#,
                order_conditions.join(" OR ")
            );

            let totals_rows = sqlx::query_as::<
                _,
                (i16, i16, i32, Option<bigdecimal::BigDecimal>, i64),
            >(&totals_query)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                eprintln!("Database error fetching order totals: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            // Map totals back to orders efficiently
            use std::collections::HashMap;
            let mut totals_map: HashMap<(i16, i16, i32), (Option<bigdecimal::BigDecimal>, i64)> =
                HashMap::new();
            for (w_id, d_id, o_id, total, count) in totals_rows {
                totals_map.insert((w_id, d_id, o_id), (total, count));
            }

            // Update orders with their totals
            for order in &mut orders {
                if let Some((total_amount, line_count)) =
                    totals_map.get(&(order.o_w_id, order.o_d_id, order.o_id))
                {
                    order.total_amount = total_amount.clone();
                    order.line_count = *line_count;
                }
            }
        }
    }

    let total_pages = ((total_count as f64) / (per_page as f64)).ceil() as u32;

    Ok(Json(OrdersListResponse {
        orders,
        total_count,
        page,
        per_page,
        total_pages,
    }))
}
