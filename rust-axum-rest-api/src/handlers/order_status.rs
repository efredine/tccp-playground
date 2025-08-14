use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

// Request Query Parameters
#[derive(Deserialize)]
pub struct OrderStatusQuery {
    pub warehouse_id: i16,
    pub district_id: i16,
    pub customer_id: i32,
}

// Response Structures
#[derive(Serialize)]
pub struct OrderStatusResponse {
    pub customer: CustomerInfo,
    pub latest_order: LatestOrderInfo,
    pub order_lines: Vec<OrderLineInfo>,
}

#[derive(Serialize)]
pub struct CustomerInfo {
    pub c_id: i32,
    pub c_first: Option<String>,
    pub c_middle: Option<String>,
    pub c_last: Option<String>,
    pub c_balance: Option<BigDecimal>,
}

#[derive(Serialize)]
pub struct LatestOrderInfo {
    pub o_id: i32,
    pub o_entry_d: Option<NaiveDateTime>,
    pub o_carrier_id: Option<i16>,
}

#[derive(Serialize)]
pub struct OrderLineInfo {
    pub ol_i_id: Option<i32>,
    pub ol_supply_w_id: Option<i16>,
    pub ol_quantity: Option<i16>,
    pub ol_amount: Option<BigDecimal>,
    pub ol_delivery_d: Option<NaiveDateTime>,
}

// Handler function
pub async fn order_status(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<OrderStatusQuery>,
) -> Result<Json<OrderStatusResponse>, StatusCode> {
    // 1. Get customer details
    let customer_row = sqlx::query!(
        r#"
        SELECT c_id, c_first, c_middle, c_last, c_balance
        FROM customer1
        WHERE c_w_id = $1 AND c_d_id = $2 AND c_id = $3
        "#,
        params.warehouse_id,
        params.district_id,
        params.customer_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching customer: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customer_info = match customer_row {
        Some(row) => CustomerInfo {
            c_id: row.c_id,
            c_first: row.c_first,
            c_middle: row.c_middle,
            c_last: row.c_last,
            c_balance: row.c_balance,
        },
        None => return Err(StatusCode::NOT_FOUND), // Customer not found
    };

    // 2. Get the latest order for the customer
    // TPC-C specification: SELECT o_id, o_carrier_id, o_entry_d FROM orders ORDER BY o_id DESC;
    let latest_order_row = sqlx::query!(
        r#"
        SELECT o_id, o_entry_d, o_carrier_id
        FROM orders1
        WHERE o_w_id = $1 AND o_d_id = $2 AND o_c_id = $3
        ORDER BY o_id DESC
        LIMIT 1
        "#,
        params.warehouse_id,
        params.district_id,
        params.customer_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching latest order: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let latest_order_info = match latest_order_row {
        Some(row) => LatestOrderInfo {
            o_id: row.o_id,
            o_entry_d: row.o_entry_d,
            o_carrier_id: row.o_carrier_id,
        },
        None => return Err(StatusCode::NOT_FOUND), // No orders found for customer
    };

    // 3. Get all order lines for the latest order
    let order_lines_rows = sqlx::query!(
        r#"
        SELECT ol_i_id, ol_supply_w_id, ol_quantity, ol_amount, ol_delivery_d
        FROM order_line1
        WHERE ol_w_id = $1 AND ol_d_id = $2 AND ol_o_id = $3
        ORDER BY ol_number ASC
        "#,
        params.warehouse_id,
        params.district_id,
        latest_order_info.o_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching order lines: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let order_lines_info: Vec<OrderLineInfo> = order_lines_rows
        .into_iter()
        .map(|row| OrderLineInfo {
            ol_i_id: row.ol_i_id,
            ol_supply_w_id: row.ol_supply_w_id,
            ol_quantity: row.ol_quantity,
            ol_amount: row.ol_amount,
            ol_delivery_d: row.ol_delivery_d,
        })
        .collect();

    Ok(Json(OrderStatusResponse {
        customer: customer_info,
        latest_order: latest_order_info,
        order_lines: order_lines_info,
    }))
}
