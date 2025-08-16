use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, QueryBuilder};
use std::collections::HashMap;

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

// Helper function to add WHERE conditions to any QueryBuilder
fn add_filter_conditions(query: &mut QueryBuilder<Postgres>, params: &OrdersQuery) -> bool {
    let mut has_conditions = false;

    if let Some(warehouse_id) = params.warehouse_id {
        if !has_conditions {
            query.push(" WHERE ");
            has_conditions = true;
        } else {
            query.push(" AND ");
        }
        query.push("o.o_w_id = ");
        query.push_bind(warehouse_id);
    }

    if let Some(district_id) = params.district_id {
        if !has_conditions {
            query.push(" WHERE ");
            has_conditions = true;
        } else {
            query.push(" AND ");
        }
        query.push("o.o_d_id = ");
        query.push_bind(district_id);
    }

    if let Some(customer_id) = params.customer_id {
        if !has_conditions {
            query.push(" WHERE ");
            has_conditions = true;
        } else {
            query.push(" AND ");
        }
        query.push("o.o_c_id = ");
        query.push_bind(customer_id);
    }

    if let Some(order_id) = params.order_id {
        if !has_conditions {
            query.push(" WHERE ");
            has_conditions = true;
        } else {
            query.push(" AND ");
        }
        query.push("o.o_id = ");
        query.push_bind(order_id);
    }

    // Parse and validate date strings properly to prevent SQL injection
    if let Some(from_date) = &params.from_date {
        // Try to parse the date string - this validates format and prevents injection
        if let Ok(parsed_date) =
            chrono::NaiveDateTime::parse_from_str(from_date, "%Y-%m-%d %H:%M:%S")
        {
            if !has_conditions {
                query.push(" WHERE ");
                has_conditions = true;
            } else {
                query.push(" AND ");
            }
            query.push("o.o_entry_d >= ");
            query.push_bind(parsed_date);
        } else if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(from_date, "%Y-%m-%d") {
            // Handle date-only format
            let datetime = parsed_date.and_hms_opt(0, 0, 0).unwrap_or_default();
            if !has_conditions {
                query.push(" WHERE ");
                has_conditions = true;
            } else {
                query.push(" AND ");
            }
            query.push("o.o_entry_d >= ");
            query.push_bind(datetime);
        }
        // Invalid date format is silently ignored (no filter applied)
    }

    if let Some(to_date) = &params.to_date {
        // Try to parse the date string - this validates format and prevents injection
        if let Ok(parsed_date) = chrono::NaiveDateTime::parse_from_str(to_date, "%Y-%m-%d %H:%M:%S")
        {
            if !has_conditions {
                query.push(" WHERE ");
                has_conditions = true;
            } else {
                query.push(" AND ");
            }
            query.push("o.o_entry_d <= ");
            query.push_bind(parsed_date);
        } else if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(to_date, "%Y-%m-%d") {
            // Handle date-only format - set to end of day
            let datetime = parsed_date.and_hms_opt(23, 59, 59).unwrap_or_default();
            if !has_conditions {
                query.push(" WHERE ");
                has_conditions = true;
            } else {
                query.push(" AND ");
            }
            query.push("o.o_entry_d <= ");
            query.push_bind(datetime);
        }
        // Invalid date format is silently ignored (no filter applied)
    }

    has_conditions
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

    // Build secure count query with parameterized conditions
    let mut count_query = QueryBuilder::new(
        "SELECT COUNT(*) FROM orders1 o LEFT JOIN customer1 c ON o.o_w_id = c.c_w_id AND o.o_d_id = c.c_d_id AND o.o_c_id = c.c_id"
    );

    // Add WHERE conditions using the shared helper function
    add_filter_conditions(&mut count_query, &params);

    let total_count_result = count_query
        .build_query_scalar::<i64>()
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error counting orders: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let total_count = total_count_result;

    // Build secure main query with parameterized conditions
    let mut main_query = QueryBuilder::new(
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
        LEFT JOIN customer1 c ON o.o_w_id = c.c_w_id AND o.o_d_id = c.c_d_id AND o.o_c_id = c.c_id"#,
    );

    // Add the same WHERE conditions using the shared helper function
    add_filter_conditions(&mut main_query, &params);

    // Add ORDER BY clause - sort_column and sort_direction are safe (from enum matching)
    main_query.push(" ORDER BY ");
    main_query.push(sort_column);
    main_query.push(" ");
    main_query.push(sort_direction);
    main_query.push(", o.o_w_id, o.o_d_id, o.o_id LIMIT ");
    main_query.push_bind(per_page as i64);
    main_query.push(" OFFSET ");
    main_query.push_bind(offset as i64);

    let orders_rows = main_query
        .build_query_as::<(
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
        )>()
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
        // Build secure totals query with parameterized conditions
        let mut totals_query = QueryBuilder::new(
            r#"
            SELECT 
                ol_w_id, 
                ol_d_id, 
                ol_o_id,
                SUM(ol_amount) as total_amount,
                COUNT(*) as line_count
            FROM order_line1 
            WHERE "#,
        );

        // Add conditions for each order using parameterized queries
        let mut separated = totals_query.separated(" OR ");
        for order in &orders {
            separated.push("(ol_w_id = ");
            separated.push_bind_unseparated(order.o_w_id);
            separated.push_unseparated(" AND ol_d_id = ");
            separated.push_bind_unseparated(order.o_d_id);
            separated.push_unseparated(" AND ol_o_id = ");
            separated.push_bind_unseparated(order.o_id);
            separated.push_unseparated(")");
        }

        totals_query.push(" GROUP BY ol_w_id, ol_d_id, ol_o_id");

        let totals_rows = totals_query
            .build_query_as::<(i16, i16, i32, Option<bigdecimal::BigDecimal>, i64)>()
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                eprintln!("Database error fetching order totals: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        // Map totals back to orders efficiently
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

    let total_pages = ((total_count as f64) / (per_page as f64)).ceil() as u32;

    Ok(Json(OrdersListResponse {
        orders,
        total_count,
        page,
        per_page,
        total_pages,
    }))
}
