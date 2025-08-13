use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Item {
    pub i_id: i32,
    pub i_im_id: Option<i32>,
    pub i_name: Option<String>,
    pub i_price: Option<BigDecimal>,
    pub i_data: Option<String>,
}
