use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct OrderLine {
    pub ol_o_id: i32,
    pub ol_d_id: i16,
    pub ol_w_id: i16,
    pub ol_number: i16,
    pub ol_i_id: Option<i32>,
    pub ol_supply_w_id: Option<i16>,
    pub ol_delivery_d: Option<DateTime<Utc>>,
    pub ol_quantity: Option<i16>,
    pub ol_amount: Option<BigDecimal>,
    pub ol_dist_info: Option<String>,
}
