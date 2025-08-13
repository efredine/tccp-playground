use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Stock {
    pub s_i_id: i32,
    pub s_w_id: i16,
    pub s_quantity: Option<i16>,
    pub s_dist_01: Option<String>,
    pub s_dist_02: Option<String>,
    pub s_dist_03: Option<String>,
    pub s_dist_04: Option<String>,
    pub s_dist_05: Option<String>,
    pub s_dist_06: Option<String>,
    pub s_dist_07: Option<String>,
    pub s_dist_08: Option<String>,
    pub s_dist_09: Option<String>,
    pub s_dist_10: Option<String>,
    pub s_ytd: Option<BigDecimal>,
    pub s_order_cnt: Option<i16>,
    pub s_remote_cnt: Option<i16>,
    pub s_data: Option<String>,
}
