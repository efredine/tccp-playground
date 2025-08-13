use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Warehouse {
    pub w_id: i16,
    pub w_name: Option<String>,
    pub w_street_1: Option<String>,
    pub w_street_2: Option<String>,
    pub w_city: Option<String>,
    pub w_state: Option<String>,
    pub w_zip: Option<String>,
    pub w_tax: Option<BigDecimal>,
    pub w_ytd: Option<BigDecimal>,
}
