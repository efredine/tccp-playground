use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct History {
    pub h_c_id: Option<i32>,
    pub h_c_d_id: Option<i16>,
    pub h_c_w_id: Option<i16>,
    pub h_d_id: Option<i16>,
    pub h_w_id: Option<i16>,
    pub h_date: Option<DateTime<Utc>>,
    pub h_amount: Option<BigDecimal>,
    pub h_data: Option<String>,
}
