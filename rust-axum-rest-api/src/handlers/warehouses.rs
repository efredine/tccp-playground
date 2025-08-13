use axum::{extract::State, http::StatusCode, Json};
use sqlx::{Pool, Postgres};

use crate::models::Warehouse;

pub async fn get_warehouses(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Warehouse>>, StatusCode> {
    let warehouses = sqlx::query_as!(
        Warehouse,
        "SELECT w_id, w_name, w_street_1, w_street_2, w_city, w_state, w_zip, w_tax, w_ytd FROM warehouse1 LIMIT 100"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(warehouses))
}
