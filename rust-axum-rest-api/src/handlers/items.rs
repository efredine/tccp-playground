use axum::{extract::{Query, State}, http::StatusCode, Json};
use serde::Deserialize;
use sqlx::{Pool, Postgres};


use crate::models::Item;

#[derive(Deserialize)]
pub struct ItemSearchQuery {
    pub warehouse_id: i16,
    pub search: Option<String>,
    pub limit: Option<i32>,
}

pub async fn search_items(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<ItemSearchQuery>,
) -> Result<Json<Vec<Item>>, StatusCode> {
    let search_term = params.search.unwrap_or_default();
    let limit = params.limit.unwrap_or(20).min(100) as i64; // Default 20, max 100 for performance
    
    // Define a temporary struct that matches the database structure
    #[derive(sqlx::FromRow)]
    struct ItemRow {
        i_id: i32,
        i_im_id: Option<i32>,
        i_name: Option<String>,
        i_price: Option<sqlx::types::BigDecimal>,
        i_data: Option<String>,
    }

    let item_rows = if search_term.is_empty() {
        // If no search term, return first N items (for initial load)
        sqlx::query_as!(
            ItemRow,
            r#"
            SELECT i_id, i_im_id, i_name, i_price, i_data
            FROM item1 
            ORDER BY i_name
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&pool)
        .await
    } else {
        // Search by item name OR item ID
        sqlx::query_as!(
            ItemRow,
            r#"
            SELECT i_id, i_im_id, i_name, i_price, i_data
            FROM item1 
            WHERE (i_name ILIKE '%' || $1 || '%' OR i_id::text = $1)
            ORDER BY 
                CASE WHEN i_id::text = $1 THEN 0 ELSE 1 END,  -- Exact ID matches first
                i_name
            LIMIT $2
            "#,
            search_term,
            limit
        )
        .fetch_all(&pool)
        .await
    }
    .map_err(|e| {
        eprintln!("Database error searching items: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Convert ItemRow to Item
    let items: Vec<Item> = item_rows
        .into_iter()
        .map(|row| Item {
            i_id: row.i_id,
            i_im_id: row.i_im_id,
            i_name: row.i_name,
            i_price: row.i_price,
            i_data: row.i_data,
        })
        .collect();

    Ok(Json(items))
}

#[derive(Deserialize)]
pub struct StockQuery {
    pub warehouse_id: i16,
    pub item_id: i32,
}

pub async fn get_stock_info(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<StockQuery>,
) -> Result<Json<StockInfo>, StatusCode> {
    let stock = sqlx::query!(
        r#"
        SELECT s_quantity, s_ytd, s_order_cnt, s_remote_cnt, s_data
        FROM stock1 
        WHERE s_w_id = $1 AND s_i_id = $2
        "#,
        params.warehouse_id,
        params.item_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error getting stock info: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match stock {
        Some(record) => Ok(Json(StockInfo {
            s_quantity: record.s_quantity.unwrap_or(0),
            s_ytd: record.s_ytd,
            s_order_cnt: record.s_order_cnt.unwrap_or(0),
            s_remote_cnt: record.s_remote_cnt.unwrap_or(0),
            s_data: record.s_data,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(serde::Serialize)]
pub struct StockInfo {
    pub s_quantity: i16,
    pub s_ytd: Option<sqlx::types::BigDecimal>,
    pub s_order_cnt: i16,
    pub s_remote_cnt: i16,
    pub s_data: Option<String>,
}