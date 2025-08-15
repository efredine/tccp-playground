use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

// Enum types for type-safe query parameters
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    OrderId,
    EntryDate,
    CustomerLast,
    WarehouseId,
    DistrictId,
    CarrierId,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc,
}

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

    // Sorting - now with proper enums!
    pub sort_by: Option<SortBy>,
    pub sort_dir: Option<SortDirection>,
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

    // Set defaults for sorting using enums
    let sort_by = params.sort_by.unwrap_or(SortBy::EntryDate);
    let sort_dir = params.sort_dir.unwrap_or(SortDirection::Desc);

    // Map sort column using pattern matching (compile-time safe!)
    let sort_column = match sort_by {
        SortBy::OrderId => "o.o_id",
        SortBy::EntryDate => "o.o_entry_d",
        SortBy::CustomerLast => "c.c_last",
        SortBy::WarehouseId => "o.o_w_id",
        SortBy::DistrictId => "o.o_d_id",
        SortBy::CarrierId => "o.o_carrier_id",
    };

    // Map sort direction using pattern matching (compile-time safe!)
    let sort_direction = match sort_dir {
        SortDirection::Asc => "ASC",
        SortDirection::Desc => "DESC",
    };

    // Build WHERE conditions based on filters (using direct values for simplicity)
    // Input is already validated through Serde deserialization
    let mut where_conditions = Vec::new();

    if let Some(warehouse_id) = params.warehouse_id {
        where_conditions.push(format!("o.o_w_id = {}", warehouse_id));
    }

    if let Some(district_id) = params.district_id {
        where_conditions.push(format!("o.o_d_id = {}", district_id));
    }

    if let Some(customer_id) = params.customer_id {
        where_conditions.push(format!("o.o_c_id = {}", customer_id));
    }

    if let Some(order_id) = params.order_id {
        where_conditions.push(format!("o.o_id = {}", order_id));
    }

    if let Some(from_date) = &params.from_date {
        // Validate date format to prevent SQL injection
        if from_date
            .chars()
            .all(|c| c.is_ascii_digit() || c == '-' || c == ' ' || c == ':' || c == 'T')
        {
            where_conditions.push(format!("o.o_entry_d >= '{}'", from_date));
        }
    }

    if let Some(to_date) = &params.to_date {
        // Validate date format to prevent SQL injection
        if to_date
            .chars()
            .all(|c| c.is_ascii_digit() || c == '-' || c == ' ' || c == ':' || c == 'T')
        {
            where_conditions.push(format!("o.o_entry_d <= '{}'", to_date));
        }
    }

    let where_clause = if where_conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_conditions.join(" AND "))
    };

    // Get the total count with the same filters
    let count_query = format!(
        "SELECT COUNT(*) FROM orders1 o LEFT JOIN customer1 c ON o.o_w_id = c.c_w_id AND o.o_d_id = c.c_d_id AND o.o_c_id = c.c_id {}",
        where_clause
    );

    let total_count_result = sqlx::query_scalar::<_, i64>(&count_query)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error counting orders: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let total_count = total_count_result;

    // Build dynamic query with filtering and sorting
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
        {}
        ORDER BY {} {}, o.o_w_id, o.o_d_id, o.o_id
        LIMIT $1 OFFSET $2
        "#,
        where_clause, sort_column, sort_direction
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
