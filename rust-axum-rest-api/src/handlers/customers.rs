use axum::{extract::{Query, State}, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use chrono::NaiveDateTime;

use crate::models::Customer;

#[derive(Deserialize)]
pub struct CustomerSearchQuery {
    pub warehouse_id: i16,
    pub district_id: i16,
    pub search: Option<String>,
    pub limit: Option<i32>,
}

pub async fn search_customers(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<CustomerSearchQuery>,
) -> Result<Json<Vec<Customer>>, StatusCode> {
    let search_term = params.search.unwrap_or_default();
    let limit = params.limit.unwrap_or(10).min(50) as i64; // Default 10, max 50 for performance, cast to i64
    
    // Define a temporary struct that matches the database structure
    #[derive(sqlx::FromRow)]
    struct CustomerRow {
        c_id: i32,
        c_d_id: i16,
        c_w_id: i16,
        c_first: Option<String>,
        c_middle: Option<String>,
        c_last: Option<String>,
        c_street_1: Option<String>,
        c_street_2: Option<String>,
        c_city: Option<String>,
        c_state: Option<String>,
        c_zip: Option<String>,
        c_phone: Option<String>,
        c_since: Option<NaiveDateTime>,
        c_credit: Option<String>,
        c_credit_lim: Option<i64>,
        c_discount: Option<sqlx::types::BigDecimal>,
        c_balance: Option<sqlx::types::BigDecimal>,
        c_ytd_payment: Option<sqlx::types::BigDecimal>,
        c_payment_cnt: Option<i16>,
        c_delivery_cnt: Option<i16>,
        c_data: Option<String>,
    }

    let customer_rows = if search_term.is_empty() {
        // If no search term, return first N customers (for initial load)
        sqlx::query_as!(
            CustomerRow,
            r#"
            SELECT c_id, c_d_id, c_w_id, c_first, c_middle, c_last, c_street_1, c_street_2, 
                   c_city, c_state, c_zip, c_phone, c_since, c_credit, c_credit_lim, 
                   c_discount, c_balance, c_ytd_payment, c_payment_cnt, c_delivery_cnt, c_data
            FROM customer1 
            WHERE c_w_id = $1 AND c_d_id = $2
            ORDER BY c_last, c_first
            LIMIT $3
            "#,
            params.warehouse_id,
            params.district_id,
            limit
        )
        .fetch_all(&pool)
        .await
    } else {
        // Search by first name OR last name
        sqlx::query_as!(
            CustomerRow,
            r#"
            SELECT c_id, c_d_id, c_w_id, c_first, c_middle, c_last, c_street_1, c_street_2, 
                   c_city, c_state, c_zip, c_phone, c_since, c_credit, c_credit_lim, 
                   c_discount, c_balance, c_ytd_payment, c_payment_cnt, c_delivery_cnt, c_data
            FROM customer1 
            WHERE c_w_id = $1 AND c_d_id = $2 
            AND (c_last ILIKE '%' || $3 || '%' OR c_first ILIKE '%' || $3 || '%')
            ORDER BY c_last, c_first
            LIMIT $4
            "#,
            params.warehouse_id,
            params.district_id,
            search_term,
            limit
        )
        .fetch_all(&pool)
        .await
    }
    .map_err(|e| {
        eprintln!("Database error searching customers: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Convert CustomerRow to Customer (handling DateTime conversion)
    let customers: Vec<Customer> = customer_rows
        .into_iter()
        .map(|row| Customer {
            c_id: row.c_id,
            c_d_id: row.c_d_id,
            c_w_id: row.c_w_id,
            c_first: row.c_first,
            c_middle: row.c_middle,
            c_last: row.c_last,
            c_street_1: row.c_street_1,
            c_street_2: row.c_street_2,
            c_city: row.c_city,
            c_state: row.c_state,
            c_zip: row.c_zip,
            c_phone: row.c_phone,
            c_since: row.c_since.map(|naive| naive.and_utc()),
            c_credit: row.c_credit,
            c_credit_lim: row.c_credit_lim,
            c_discount: row.c_discount,
            c_balance: row.c_balance,
            c_ytd_payment: row.c_ytd_payment,
            c_payment_cnt: row.c_payment_cnt,
            c_delivery_cnt: row.c_delivery_cnt,
            c_data: row.c_data,
        })
        .collect();

    Ok(Json(customers))
}