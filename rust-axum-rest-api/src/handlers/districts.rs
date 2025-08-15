use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::models::District;

#[derive(Deserialize)]
pub struct DistrictsQuery {
    pub warehouse_id: i16,
}

pub async fn get_districts(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<DistrictsQuery>,
) -> Result<Json<Vec<District>>, StatusCode> {
    let districts = sqlx::query_as!(
        District,
        "SELECT d_id, d_w_id, d_name, d_street_1, d_street_2, d_city, d_state, d_zip, d_tax, d_ytd, d_next_o_id FROM district1 WHERE d_w_id = $1 ORDER BY d_id ASC",
        params.warehouse_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error fetching districts: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(districts))
}
