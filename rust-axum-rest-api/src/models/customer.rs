use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Customer {
    pub c_id: i32,
    pub c_d_id: i16,
    pub c_w_id: i16,
    pub c_first: Option<String>,
    pub c_middle: Option<String>,
    pub c_last: Option<String>,
    pub c_street_1: Option<String>,
    pub c_street_2: Option<String>,
    pub c_city: Option<String>,
    pub c_state: Option<String>,
    pub c_zip: Option<String>,
    pub c_phone: Option<String>,
    pub c_since: Option<DateTime<Utc>>,
    pub c_credit: Option<String>,
    pub c_credit_lim: Option<i64>,
    pub c_discount: Option<BigDecimal>,
    pub c_balance: Option<BigDecimal>,
    pub c_ytd_payment: Option<BigDecimal>,
    pub c_payment_cnt: Option<i16>,
    pub c_delivery_cnt: Option<i16>,
    pub c_data: Option<String>,
}
