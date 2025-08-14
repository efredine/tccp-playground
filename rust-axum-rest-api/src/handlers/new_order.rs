use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Transaction};


// Request Structure
#[derive(Deserialize)]
pub struct NewOrderRequest {
    pub warehouse_id: i16,
    pub district_id: i16,
    pub customer_id: i32,
    pub order_lines: Vec<OrderLineRequest>,
}

#[derive(Deserialize)]
pub struct OrderLineRequest {
    pub item_id: i32,
    pub supply_warehouse_id: i16,
    pub quantity: i16,
}

// Response Structures
#[derive(Serialize)]
pub struct NewOrderResponse {
    pub order_id: i32,
    pub customer: CustomerSummary,
    pub warehouse_tax: BigDecimal,
    pub district_tax: BigDecimal,
    pub order_entry_date: NaiveDateTime,
    pub total_amount: BigDecimal,
    pub order_lines: Vec<OrderLineSummary>,
}

#[derive(Serialize)]
pub struct CustomerSummary {
    pub customer_id: i32,
    pub last_name: String,
    pub credit: String,
    pub discount: BigDecimal,
}

#[derive(Serialize)]
pub struct OrderLineSummary {
    pub item_id: i32,
    pub supply_warehouse_id: i16,
    pub quantity: i16,
    pub item_name: String,
    pub item_price: BigDecimal,
    pub stock_quantity: i16,
    pub brand_generic: String,
    pub line_amount: BigDecimal,
}

// Internal structures for database operations
struct WarehouseData {
    w_tax: BigDecimal,
}

#[allow(dead_code)]
struct DistrictData {
    d_tax: BigDecimal,
    d_next_o_id: i32,
}

struct CustomerData {
    c_last: String,
    c_credit: String,
    c_discount: BigDecimal,
}

struct ItemData {
    i_name: String,
    i_price: BigDecimal,
}

#[allow(dead_code)]
struct StockData {
    s_quantity: i16,
    s_dist_info: String,
    s_ytd: BigDecimal,
    s_order_cnt: i16,
    s_remote_cnt: i16,
    s_data: String,
}

// Handler function
pub async fn new_order(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<NewOrderRequest>,
) -> Result<Json<NewOrderResponse>, StatusCode> {
    // Start transaction - TPC-C New Order is a complex multi-table transaction
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| {
            eprintln!("Failed to start transaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Validate request
    if request.order_lines.is_empty() || request.order_lines.len() > 15 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let entry_date = Utc::now().naive_utc();

    // Step 1: Get warehouse data and validate warehouse exists
    let warehouse = get_warehouse_data(&mut tx, request.warehouse_id).await?;

    // Step 2: Get district data and update next order ID
    let (district, order_id) = get_and_update_district(&mut tx, request.warehouse_id, request.district_id).await?;

    // Step 3: Get customer data
    let customer = get_customer_data(&mut tx, request.warehouse_id, request.district_id, request.customer_id).await?;

    // Step 4: Insert new order record
    insert_new_order(&mut tx, request.warehouse_id, request.district_id, order_id, request.customer_id, entry_date, &request.order_lines).await?;

    // Step 5: Process each order line
    let mut order_line_summaries = Vec::new();
    let mut total_amount = BigDecimal::from_f64(0.0).unwrap();
    let mut all_local = true;

    for (line_number, order_line) in request.order_lines.iter().enumerate() {
        // Check if this is a remote order line
        if order_line.supply_warehouse_id != request.warehouse_id {
            all_local = false;
        }

        // Get item data
        let item = get_item_data(&mut tx, order_line.item_id).await?;

        // Get and update stock data
        let stock = get_and_update_stock(
            &mut tx,
            order_line.item_id,
            order_line.supply_warehouse_id,
            order_line.quantity,
            request.district_id,
            order_line.supply_warehouse_id != request.warehouse_id,
        ).await?;

        // Calculate line amount
        let line_amount = &item.i_price * BigDecimal::from(order_line.quantity);
        total_amount += &line_amount;

        // Determine brand/generic indicator
        let brand_generic = if item.i_name.contains("ORIGINAL") && stock.s_data.contains("ORIGINAL") {
            "B".to_string()
        } else {
            "G".to_string()
        };

        // Insert order line
        insert_order_line(
            &mut tx,
            OrderLineParams {
                warehouse_id: request.warehouse_id,
                district_id: request.district_id,
                order_id,
                line_number: (line_number + 1) as i16,
                item_id: order_line.item_id,
                supply_warehouse_id: order_line.supply_warehouse_id,
                quantity: order_line.quantity,
                amount: line_amount.clone(),
                dist_info: stock.s_dist_info.clone(),
            },
        ).await?;

        order_line_summaries.push(OrderLineSummary {
            item_id: order_line.item_id,
            supply_warehouse_id: order_line.supply_warehouse_id,
            quantity: order_line.quantity,
            item_name: item.i_name,
            item_price: item.i_price,
            stock_quantity: stock.s_quantity,
            brand_generic,
            line_amount,
        });
    }

    // Apply taxes and discount
    let tax_rate = (&warehouse.w_tax + &district.d_tax) * &total_amount;
    let discount_amount = &customer.c_discount * &total_amount;
    total_amount = total_amount + tax_rate - discount_amount;

    // Update order with final details
    update_order_totals(&mut tx, request.warehouse_id, request.district_id, order_id, request.order_lines.len() as i16, all_local).await?;

    // Commit transaction
    tx.commit()
        .await
        .map_err(|e| {
            eprintln!("Failed to commit transaction: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(NewOrderResponse {
        order_id,
        customer: CustomerSummary {
            customer_id: request.customer_id,
            last_name: customer.c_last,
            credit: customer.c_credit,
            discount: customer.c_discount,
        },
        warehouse_tax: warehouse.w_tax,
        district_tax: district.d_tax,
        order_entry_date: entry_date,
        total_amount,
        order_lines: order_line_summaries,
    }))
}

// Database helper functions
async fn get_warehouse_data(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i16,
) -> Result<WarehouseData, StatusCode> {
    let row = sqlx::query!(
        "SELECT w_tax FROM warehouse1 WHERE w_id = $1",
        warehouse_id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching warehouse: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match row {
        Some(row) => Ok(WarehouseData {
            w_tax: row.w_tax.unwrap_or_else(|| BigDecimal::from_f64(0.0).unwrap()),
        }),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn get_and_update_district(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i16,
    district_id: i16,
) -> Result<(DistrictData, i32), StatusCode> {
    // Get current district data
    let row = sqlx::query!(
        "SELECT d_tax, d_next_o_id FROM district1 WHERE d_w_id = $1 AND d_id = $2",
        warehouse_id,
        district_id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching district: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let district_row = match row {
        Some(row) => row,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let next_order_id = district_row.d_next_o_id.unwrap_or(1);

    // Update district with incremented order ID
    sqlx::query!(
        "UPDATE district1 SET d_next_o_id = $1 WHERE d_w_id = $2 AND d_id = $3",
        next_order_id + 1,
        warehouse_id,
        district_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error updating district: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        DistrictData {
            d_tax: district_row.d_tax.unwrap_or_else(|| BigDecimal::from_f64(0.0).unwrap()),
            d_next_o_id: next_order_id,
        },
        next_order_id,
    ))
}

async fn get_customer_data(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i16,
    district_id: i16,
    customer_id: i32,
) -> Result<CustomerData, StatusCode> {
    let row = sqlx::query!(
        "SELECT c_last, c_credit, c_discount FROM customer1 WHERE c_w_id = $1 AND c_d_id = $2 AND c_id = $3",
        warehouse_id,
        district_id,
        customer_id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching customer: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match row {
        Some(row) => Ok(CustomerData {
            c_last: row.c_last.unwrap_or_default(),
            c_credit: row.c_credit.unwrap_or_default(),
            c_discount: row.c_discount.unwrap_or_else(|| BigDecimal::from_f64(0.0).unwrap()),
        }),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn insert_new_order(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i16,
    district_id: i16,
    order_id: i32,
    customer_id: i32,
    entry_date: NaiveDateTime,
    order_lines: &[OrderLineRequest],
) -> Result<(), StatusCode> {
    // Insert into orders table
    sqlx::query!(
        r#"
        INSERT INTO orders1 (o_id, o_d_id, o_w_id, o_c_id, o_entry_d, o_carrier_id, o_ol_cnt, o_all_local)
        VALUES ($1, $2, $3, $4, $5, NULL, $6, NULL)
        "#,
        order_id,
        district_id,
        warehouse_id,
        customer_id,
        entry_date,
        order_lines.len() as i16
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error inserting order: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Insert into new_orders table
    sqlx::query!(
        "INSERT INTO new_orders1 (no_o_id, no_d_id, no_w_id) VALUES ($1, $2, $3)",
        order_id,
        district_id,
        warehouse_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error inserting new order: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}

async fn get_item_data(
    tx: &mut Transaction<'_, Postgres>,
    item_id: i32,
) -> Result<ItemData, StatusCode> {
    let row = sqlx::query!(
        "SELECT i_name, i_price FROM item1 WHERE i_id = $1",
        item_id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching item: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match row {
        Some(row) => Ok(ItemData {
            i_name: row.i_name.unwrap_or_default(),
            i_price: row.i_price.unwrap_or_else(|| BigDecimal::from_f64(0.0).unwrap()),
        }),
        None => Err(StatusCode::NOT_FOUND), // TPC-C: 1% of items should be invalid
    }
}

async fn get_and_update_stock(
    tx: &mut Transaction<'_, Postgres>,
    item_id: i32,
    warehouse_id: i16,
    quantity: i16,
    district_id: i16,
    is_remote: bool,
) -> Result<StockData, StatusCode> {
    // Get current stock data
    let row = sqlx::query!(
        r#"
        SELECT s_quantity, s_dist_01, s_dist_02, s_dist_03, s_dist_04, s_dist_05,
               s_dist_06, s_dist_07, s_dist_08, s_dist_09, s_dist_10,
               s_ytd, s_order_cnt, s_remote_cnt, s_data
        FROM stock1 WHERE s_i_id = $1 AND s_w_id = $2
        "#,
        item_id,
        warehouse_id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching stock: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let stock_row = match row {
        Some(row) => row,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let current_quantity = stock_row.s_quantity.unwrap_or(0);
    let new_quantity = if current_quantity >= quantity {
        current_quantity - quantity
    } else {
        current_quantity - quantity + 91 // TPC-C specification
    };

    // Get the appropriate district info
    let district_info = match district_id {
        1 => stock_row.s_dist_01,
        2 => stock_row.s_dist_02,
        3 => stock_row.s_dist_03,
        4 => stock_row.s_dist_04,
        5 => stock_row.s_dist_05,
        6 => stock_row.s_dist_06,
        7 => stock_row.s_dist_07,
        8 => stock_row.s_dist_08,
        9 => stock_row.s_dist_09,
        10 => stock_row.s_dist_10,
        _ => None,
    }.unwrap_or_default();

    // Update stock
    let new_ytd = stock_row.s_ytd.unwrap_or_else(|| BigDecimal::from_f64(0.0).unwrap()) + BigDecimal::from(quantity);
    let new_order_cnt = stock_row.s_order_cnt.unwrap_or(0) + 1;
    let new_remote_cnt = if is_remote {
        stock_row.s_remote_cnt.unwrap_or(0) + 1
    } else {
        stock_row.s_remote_cnt.unwrap_or(0)
    };

    sqlx::query!(
        r#"
        UPDATE stock1 
        SET s_quantity = $1, s_ytd = $2, s_order_cnt = $3, s_remote_cnt = $4
        WHERE s_i_id = $5 AND s_w_id = $6
        "#,
        new_quantity,
        new_ytd,
        new_order_cnt,
        new_remote_cnt,
        item_id,
        warehouse_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error updating stock: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StockData {
        s_quantity: new_quantity,
        s_dist_info: district_info,
        s_ytd: new_ytd,
        s_order_cnt: new_order_cnt,
        s_remote_cnt: new_remote_cnt,
        s_data: stock_row.s_data.unwrap_or_default(),
    })
}

struct OrderLineParams {
    warehouse_id: i16,
    district_id: i16,
    order_id: i32,
    line_number: i16,
    item_id: i32,
    supply_warehouse_id: i16,
    quantity: i16,
    amount: BigDecimal,
    dist_info: String,
}

async fn insert_order_line(
    tx: &mut Transaction<'_, Postgres>,
    params: OrderLineParams,
) -> Result<(), StatusCode> {
    sqlx::query!(
        r#"
        INSERT INTO order_line1 (ol_o_id, ol_d_id, ol_w_id, ol_number, ol_i_id, 
                                ol_supply_w_id, ol_delivery_d, ol_quantity, ol_amount, ol_dist_info)
        VALUES ($1, $2, $3, $4, $5, $6, NULL, $7, $8, $9)
        "#,
        params.order_id,
        params.district_id,
        params.warehouse_id,
        params.line_number,
        params.item_id,
        params.supply_warehouse_id,
        params.quantity,
        params.amount,
        params.dist_info
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error inserting order line: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}

async fn update_order_totals(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i16,
    district_id: i16,
    order_id: i32,
    line_count: i16,
    all_local: bool,
) -> Result<(), StatusCode> {
    sqlx::query!(
        "UPDATE orders1 SET o_ol_cnt = $1, o_all_local = $2 WHERE o_w_id = $3 AND o_d_id = $4 AND o_id = $5",
        line_count,
        if all_local { 1i16 } else { 0i16 },
        warehouse_id,
        district_id,
        order_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error updating order totals: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}