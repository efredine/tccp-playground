use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
#[derive(Deserialize)]
pub struct StockLevelQuery {
    pub warehouse_id: i16,
    pub district_id: i16,
    pub threshold: i16,
}

#[derive(Serialize)]
pub struct StockLevelResponse {
    pub warehouse_id: i16,
    pub district_id: i16,
    pub threshold: i16,
    pub low_stock_count: i64,
}

pub async fn stock_level(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<StockLevelQuery>,
) -> Result<Json<StockLevelResponse>, StatusCode> {
    // First, get the district's next order ID
    let d_next_o_id_result = sqlx::query_scalar!(
        "SELECT d_next_o_id FROM district1 WHERE d_id = $1 AND d_w_id = $2",
        params.district_id,
        params.warehouse_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let d_next_o_id = match d_next_o_id_result {
        Some(Some(id)) => id,
        _ => return Err(StatusCode::NOT_FOUND),
    };

    // This implements the TPC-C stock level transaction using the specification approach
    // SELECT COUNT(DISTINCT (s_i_id))
    // FROM order_line, stock
    // WHERE ol_w_id=:w_id AND ol_d_id=:d_id AND ol_o_id<:o_id AND ol_o_id>=:o_id-20
    //   AND s_w_id=:w_id AND s_i_id=ol_i_id AND s_quantity < :threshold
    let low_stock_count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(DISTINCT (s_i_id)) as "count!"
        FROM order_line1 ol, stock1 s
        WHERE ol.ol_w_id = $1 
          AND ol.ol_d_id = $2
          AND ol.ol_o_id < $3 
          AND ol.ol_o_id >= $4
          AND s.s_w_id = $1
          AND s.s_i_id = ol.ol_i_id 
          AND s.s_quantity < $5
        "#,
        params.warehouse_id,
        params.district_id,
        d_next_o_id,
        d_next_o_id - 20,
        params.threshold
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(StockLevelResponse {
        warehouse_id: params.warehouse_id,
        district_id: params.district_id,
        threshold: params.threshold,
        low_stock_count,
    }))
}
