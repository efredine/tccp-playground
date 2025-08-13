use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct District {
    pub d_id: i16,
    pub d_w_id: i16,
    pub d_name: Option<String>,
    pub d_street_1: Option<String>,
    pub d_street_2: Option<String>,
    pub d_city: Option<String>,
    pub d_state: Option<String>,
    pub d_zip: Option<String>,
    pub d_tax: Option<BigDecimal>,
    pub d_ytd: Option<BigDecimal>,
    pub d_next_o_id: Option<i32>,
}
