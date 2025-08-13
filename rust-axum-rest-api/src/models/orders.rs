use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Orders {
    pub o_id: i32,
    pub o_d_id: i16,
    pub o_w_id: i16,
    pub o_c_id: Option<i32>,
    pub o_entry_d: Option<DateTime<Utc>>,
    pub o_carrier_id: Option<i16>,
    pub o_ol_cnt: Option<i16>,
    pub o_all_local: Option<i16>,
}
