use axum::{extract::State, http::StatusCode, Json};
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Transaction};

// Request Structure
#[derive(Deserialize)]
pub struct DeliveryRequest {
    pub warehouse_id: i16,
    pub district_id: i16,
}

// Response Structures
#[derive(Serialize)]
pub struct DeliveryResponse {
    pub warehouse_id: i16,
    pub district_id: i16,
    pub delivery_date: NaiveDateTime,
    pub delivered_orders: Vec<DeliveredOrder>,
    pub total_orders_delivered: usize,
}

#[derive(Serialize, Clone)]
pub struct DeliveredOrder {
    pub order_id: i32,
    pub customer_id: i32,
    pub carrier_id: i16,
    pub order_line_count: usize,
    pub total_amount: BigDecimal,
}

// Internal structures for database operations
#[allow(dead_code)]
struct NewOrderRecord {
    no_o_id: i32,
}

#[allow(dead_code)]
struct OrderInfo {
    o_c_id: i32,
}

#[allow(dead_code)]
struct OrderLineInfo {
    ol_amount: BigDecimal,
}

// Handler function
pub async fn delivery(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<DeliveryRequest>,
) -> Result<Json<DeliveryResponse>, StatusCode> {
    let delivery_date = Utc::now().naive_utc();
    let carrier_id: i16 = 1; // TPC-C uses random carrier_id 1-10, we'll use 1 for simplicity

    // Start transaction - TPC-C Delivery processes multiple orders atomically
    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("Failed to start transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // TPC-C Delivery processes one order per district, but for simplicity
    // we'll process orders for the specified district only
    let mut delivered_orders = Vec::new();

    // Step 1: Find the oldest undelivered order for this district
    if let Some(delivered_order) = process_district_delivery(
        &mut tx,
        request.warehouse_id,
        request.district_id,
        carrier_id,
        delivery_date,
    )
    .await?
    {
        delivered_orders.push(delivered_order);
    }

    // Commit transaction
    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(DeliveryResponse {
        warehouse_id: request.warehouse_id,
        district_id: request.district_id,
        delivery_date,
        delivered_orders: delivered_orders.clone(),
        total_orders_delivered: delivered_orders.len(),
    }))
}

// Process delivery for a single district
async fn process_district_delivery(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i16,
    district_id: i16,
    carrier_id: i16,
    delivery_date: NaiveDateTime,
) -> Result<Option<DeliveredOrder>, StatusCode> {
    // Step 1: Find the oldest undelivered order (smallest order ID in new_orders)
    let new_order_row = sqlx::query!(
        r#"
        SELECT no_o_id
        FROM new_orders1
        WHERE no_w_id = $1 AND no_d_id = $2
        ORDER BY no_o_id ASC
        LIMIT 1
        "#,
        warehouse_id,
        district_id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error finding new order: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let order_id = match new_order_row {
        Some(row) => row.no_o_id,
        None => return Ok(None), // No undelivered orders for this district
    };

    // Step 2: Get order information (customer_id)
    let order_row = sqlx::query!(
        "SELECT o_c_id FROM orders1 WHERE o_w_id = $1 AND o_d_id = $2 AND o_id = $3",
        warehouse_id,
        district_id,
        order_id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching order: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customer_id = match order_row {
        Some(row) => row.o_c_id.unwrap_or(0),
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR), // Order should exist
    };

    // Step 3: Update the order with carrier_id
    sqlx::query!(
        "UPDATE orders1 SET o_carrier_id = $1 WHERE o_w_id = $2 AND o_d_id = $3 AND o_id = $4",
        carrier_id,
        warehouse_id,
        district_id,
        order_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error updating order: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Step 4: Update all order lines with delivery date and get total amount
    let order_lines_rows = sqlx::query!(
        r#"
        SELECT ol_amount
        FROM order_line1
        WHERE ol_w_id = $1 AND ol_d_id = $2 AND ol_o_id = $3
        "#,
        warehouse_id,
        district_id,
        order_id
    )
    .fetch_all(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching order lines: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Calculate total amount and count order lines
    let mut total_amount = BigDecimal::from_f64(0.0).unwrap();
    let order_line_count = order_lines_rows.len();

    for row in &order_lines_rows {
        if let Some(amount) = &row.ol_amount {
            total_amount += amount;
        }
    }

    // Update order lines with delivery date
    sqlx::query!(
        r#"
        UPDATE order_line1 
        SET ol_delivery_d = $1
        WHERE ol_w_id = $2 AND ol_d_id = $3 AND ol_o_id = $4
        "#,
        delivery_date,
        warehouse_id,
        district_id,
        order_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error updating order lines: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Step 5: Update customer balance and delivery count
    sqlx::query!(
        r#"
        UPDATE customer1 
        SET c_balance = c_balance + $1, c_delivery_cnt = c_delivery_cnt + 1
        WHERE c_w_id = $2 AND c_d_id = $3 AND c_id = $4
        "#,
        total_amount,
        warehouse_id,
        district_id,
        customer_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error updating customer: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Step 6: Remove the order from new_orders (it's now delivered)
    sqlx::query!(
        "DELETE FROM new_orders1 WHERE no_w_id = $1 AND no_d_id = $2 AND no_o_id = $3",
        warehouse_id,
        district_id,
        order_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error removing from new_orders: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Some(DeliveredOrder {
        order_id,
        customer_id,
        carrier_id,
        order_line_count,
        total_amount,
    }))
}
