use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct NewOrders {
    pub no_o_id: i32,
    pub no_d_id: i16,
    pub no_w_id: i16,
}
