use axum::{extract::State, http::StatusCode, Json};
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, Transaction};

// Request Structure
#[derive(Deserialize)]
pub struct PaymentRequest {
    pub warehouse_id: i16,
    pub district_id: i16,
    pub customer_id: i32,
    pub amount: f64,
}

// Response Structures
#[derive(Serialize)]
pub struct PaymentResponse {
    pub warehouse: WarehouseInfo,
    pub district: DistrictInfo,
    pub customer: PaymentCustomerInfo,
    pub payment_date: NaiveDateTime,
    pub payment_amount: BigDecimal,
}

#[derive(Serialize)]
pub struct WarehouseInfo {
    pub w_id: i16,
    pub w_name: String,
    pub w_street_1: String,
    pub w_street_2: String,
    pub w_city: String,
    pub w_state: String,
    pub w_zip: String,
}

#[derive(Serialize)]
pub struct DistrictInfo {
    pub d_id: i16,
    pub d_name: String,
    pub d_street_1: String,
    pub d_street_2: String,
    pub d_city: String,
    pub d_state: String,
    pub d_zip: String,
}

#[derive(Serialize)]
pub struct PaymentCustomerInfo {
    pub c_id: i32,
    pub c_first: String,
    pub c_middle: String,
    pub c_last: String,
    pub c_street_1: String,
    pub c_street_2: String,
    pub c_city: String,
    pub c_state: String,
    pub c_zip: String,
    pub c_phone: String,
    pub c_since: NaiveDateTime,
    pub c_credit: String,
    pub c_credit_lim: i64,
    pub c_discount: BigDecimal,
    pub c_balance: BigDecimal,
}

// Internal data structures
#[allow(dead_code)]
struct WarehouseData {
    w_name: String,
    w_street_1: String,
    w_street_2: String,
    w_city: String,
    w_state: String,
    w_zip: String,
    w_ytd: BigDecimal,
}

#[allow(dead_code)]
struct DistrictData {
    d_name: String,
    d_street_1: String,
    d_street_2: String,
    d_city: String,
    d_state: String,
    d_zip: String,
    d_ytd: BigDecimal,
}

#[allow(dead_code)]
struct CustomerData {
    c_first: String,
    c_middle: String,
    c_last: String,
    c_street_1: String,
    c_street_2: String,
    c_city: String,
    c_state: String,
    c_zip: String,
    c_phone: String,
    c_since: NaiveDateTime,
    c_credit: String,
    c_credit_lim: i64,
    c_discount: BigDecimal,
    c_balance: BigDecimal,
    c_ytd_payment: BigDecimal,
    c_payment_cnt: i16,
    c_data: String,
}

// Handler function
pub async fn payment(
    State(pool): State<Pool<Postgres>>,
    Json(request): Json<PaymentRequest>,
) -> Result<Json<PaymentResponse>, StatusCode> {
    // Convert payment amount to BigDecimal for precise calculations
    let payment_amount = BigDecimal::from_f64(request.amount).ok_or(StatusCode::BAD_REQUEST)?;

    if payment_amount <= BigDecimal::from(0) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let payment_date = Utc::now().naive_utc();

    // Start transaction - TPC-C Payment is a multi-table transaction
    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("Failed to start transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Step 1: Get and update warehouse data
    let warehouse =
        get_and_update_warehouse(&mut tx, request.warehouse_id, &payment_amount).await?;

    // Step 2: Get and update district data
    let district = get_and_update_district(
        &mut tx,
        request.warehouse_id,
        request.district_id,
        &payment_amount,
    )
    .await?;

    // Step 3: Get and update customer data
    let customer = get_and_update_customer(
        &mut tx,
        request.warehouse_id,
        request.district_id,
        request.customer_id,
        &payment_amount,
    )
    .await?;

    // Step 4: Insert history record
    insert_payment_history(
        &mut tx,
        PaymentHistoryParams {
            warehouse_id: request.warehouse_id,
            district_id: request.district_id,
            customer_id: request.customer_id,
            payment_amount: payment_amount.clone(),
            payment_date,
            warehouse_name: warehouse.w_name.clone(),
            district_name: district.d_name.clone(),
        },
    )
    .await?;

    // Commit transaction
    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(PaymentResponse {
        warehouse: WarehouseInfo {
            w_id: request.warehouse_id,
            w_name: warehouse.w_name,
            w_street_1: warehouse.w_street_1,
            w_street_2: warehouse.w_street_2,
            w_city: warehouse.w_city,
            w_state: warehouse.w_state,
            w_zip: warehouse.w_zip,
        },
        district: DistrictInfo {
            d_id: request.district_id,
            d_name: district.d_name,
            d_street_1: district.d_street_1,
            d_street_2: district.d_street_2,
            d_city: district.d_city,
            d_state: district.d_state,
            d_zip: district.d_zip,
        },
        customer: PaymentCustomerInfo {
            c_id: request.customer_id,
            c_first: customer.c_first,
            c_middle: customer.c_middle,
            c_last: customer.c_last,
            c_street_1: customer.c_street_1,
            c_street_2: customer.c_street_2,
            c_city: customer.c_city,
            c_state: customer.c_state,
            c_zip: customer.c_zip,
            c_phone: customer.c_phone,
            c_since: customer.c_since,
            c_credit: customer.c_credit,
            c_credit_lim: customer.c_credit_lim,
            c_discount: customer.c_discount,
            c_balance: customer.c_balance,
        },
        payment_date,
        payment_amount,
    }))
}

// Database helper functions
async fn get_and_update_warehouse(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i16,
    payment_amount: &BigDecimal,
) -> Result<WarehouseData, StatusCode> {
    // Get current warehouse data
    let row = sqlx::query!(
        r#"
        SELECT w_name, w_street_1, w_street_2, w_city, w_state, w_zip, w_ytd
        FROM warehouse1 WHERE w_id = $1
        "#,
        warehouse_id
    )
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching warehouse: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let warehouse_row = match row {
        Some(row) => row,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let current_ytd = warehouse_row.w_ytd.unwrap_or_else(|| BigDecimal::from(0));
    let new_ytd = &current_ytd + payment_amount;

    // Update warehouse YTD
    sqlx::query!(
        "UPDATE warehouse1 SET w_ytd = $1 WHERE w_id = $2",
        new_ytd,
        warehouse_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error updating warehouse: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(WarehouseData {
        w_name: warehouse_row.w_name.unwrap_or_default(),
        w_street_1: warehouse_row.w_street_1.unwrap_or_default(),
        w_street_2: warehouse_row.w_street_2.unwrap_or_default(),
        w_city: warehouse_row.w_city.unwrap_or_default(),
        w_state: warehouse_row.w_state.unwrap_or_default(),
        w_zip: warehouse_row.w_zip.unwrap_or_default(),
        w_ytd: new_ytd,
    })
}

async fn get_and_update_district(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i16,
    district_id: i16,
    payment_amount: &BigDecimal,
) -> Result<DistrictData, StatusCode> {
    // Get current district data
    let row = sqlx::query!(
        r#"
        SELECT d_name, d_street_1, d_street_2, d_city, d_state, d_zip, d_ytd
        FROM district1 WHERE d_w_id = $1 AND d_id = $2
        "#,
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

    let current_ytd = district_row.d_ytd.unwrap_or_else(|| BigDecimal::from(0));
    let new_ytd = &current_ytd + payment_amount;

    // Update district YTD
    sqlx::query!(
        "UPDATE district1 SET d_ytd = $1 WHERE d_w_id = $2 AND d_id = $3",
        new_ytd,
        warehouse_id,
        district_id
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error updating district: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(DistrictData {
        d_name: district_row.d_name.unwrap_or_default(),
        d_street_1: district_row.d_street_1.unwrap_or_default(),
        d_street_2: district_row.d_street_2.unwrap_or_default(),
        d_city: district_row.d_city.unwrap_or_default(),
        d_state: district_row.d_state.unwrap_or_default(),
        d_zip: district_row.d_zip.unwrap_or_default(),
        d_ytd: new_ytd,
    })
}

async fn get_and_update_customer(
    tx: &mut Transaction<'_, Postgres>,
    warehouse_id: i16,
    district_id: i16,
    customer_id: i32,
    payment_amount: &BigDecimal,
) -> Result<CustomerData, StatusCode> {
    // Get current customer data
    let row = sqlx::query!(
        r#"
        SELECT c_first, c_middle, c_last, c_street_1, c_street_2, c_city, c_state, c_zip,
               c_phone, c_since, c_credit, c_credit_lim, c_discount, c_balance, 
               c_ytd_payment, c_payment_cnt, c_data
        FROM customer1 WHERE c_w_id = $1 AND c_d_id = $2 AND c_id = $3
        "#,
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

    let customer_row = match row {
        Some(row) => row,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Calculate new values
    let current_balance = customer_row
        .c_balance
        .unwrap_or_else(|| BigDecimal::from(0));
    let new_balance = &current_balance - payment_amount; // Payment decreases balance

    let current_ytd_payment = customer_row
        .c_ytd_payment
        .unwrap_or_else(|| BigDecimal::from(0));
    let new_ytd_payment = &current_ytd_payment + payment_amount;

    let current_payment_cnt = customer_row.c_payment_cnt.unwrap_or(0);
    let new_payment_cnt = current_payment_cnt + 1;

    // Handle credit data update for "BC" (Bad Credit) customers
    let credit = customer_row.c_credit.as_deref().unwrap_or("");
    let new_c_data = if credit == "BC" {
        // For bad credit customers, prepend payment info to c_data (TPC-C requirement)
        let payment_info = format!(
            "{}|{}|{}|{}|{}|{}|{}|",
            customer_id,
            district_id,
            warehouse_id,
            district_id,
            warehouse_id,
            payment_amount,
            payment_amount
        );
        let existing_data = customer_row.c_data.unwrap_or_default();
        let combined = format!("{}{}", payment_info, existing_data);
        // Truncate to maximum c_data length (500 chars typically)
        if combined.len() > 500 {
            combined[..500].to_string()
        } else {
            combined
        }
    } else {
        customer_row.c_data.unwrap_or_default()
    };

    // Update customer
    sqlx::query!(
        r#"
        UPDATE customer1 
        SET c_balance = $1, c_ytd_payment = $2, c_payment_cnt = $3, c_data = $4
        WHERE c_w_id = $5 AND c_d_id = $6 AND c_id = $7
        "#,
        new_balance,
        new_ytd_payment,
        new_payment_cnt,
        new_c_data,
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

    Ok(CustomerData {
        c_first: customer_row.c_first.unwrap_or_default(),
        c_middle: customer_row.c_middle.unwrap_or_default(),
        c_last: customer_row.c_last.unwrap_or_default(),
        c_street_1: customer_row.c_street_1.unwrap_or_default(),
        c_street_2: customer_row.c_street_2.unwrap_or_default(),
        c_city: customer_row.c_city.unwrap_or_default(),
        c_state: customer_row.c_state.unwrap_or_default(),
        c_zip: customer_row.c_zip.unwrap_or_default(),
        c_phone: customer_row.c_phone.unwrap_or_default(),
        c_since: customer_row.c_since.unwrap_or_default(),
        c_credit: customer_row.c_credit.unwrap_or_default(),
        c_credit_lim: customer_row.c_credit_lim.unwrap_or(0),
        c_discount: customer_row
            .c_discount
            .unwrap_or_else(|| BigDecimal::from(0)),
        c_balance: new_balance,
        c_ytd_payment: new_ytd_payment,
        c_payment_cnt: new_payment_cnt,
        c_data: new_c_data,
    })
}

struct PaymentHistoryParams {
    warehouse_id: i16,
    district_id: i16,
    customer_id: i32,
    payment_amount: BigDecimal,
    payment_date: NaiveDateTime,
    warehouse_name: String,
    district_name: String,
}

async fn insert_payment_history(
    tx: &mut Transaction<'_, Postgres>,
    params: PaymentHistoryParams,
) -> Result<(), StatusCode> {
    // Create history data string (TPC-C format)
    let h_data = format!("{} {}", params.warehouse_name, params.district_name);

    sqlx::query!(
        r#"
        INSERT INTO history1 (h_c_id, h_c_d_id, h_c_w_id, h_d_id, h_w_id, h_date, h_amount, h_data)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        params.customer_id,
        params.district_id,
        params.warehouse_id,
        params.district_id,
        params.warehouse_id,
        params.payment_date,
        params.payment_amount,
        h_data
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        eprintln!("Database error inserting payment history: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(())
}
