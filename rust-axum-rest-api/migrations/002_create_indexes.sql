-- TPC-C Indexes
-- Based on sysbench TPC-C index creation

CREATE INDEX IF NOT EXISTS idx_customer1 ON customer1 (c_w_id,c_d_id,c_last,c_first);
CREATE INDEX IF NOT EXISTS idx_orders1 ON orders1 (o_w_id,o_d_id,o_c_id,o_id);
CREATE INDEX IF NOT EXISTS fkey_stock_21 ON stock1 (s_i_id);
CREATE INDEX IF NOT EXISTS fkey_order_line_21 ON order_line1 (ol_supply_w_id,ol_i_id);
CREATE INDEX IF NOT EXISTS fkey_history_11 ON history1 (h_c_w_id,h_c_d_id,h_c_id);
CREATE INDEX IF NOT EXISTS fkey_history_21 ON history1 (h_w_id,h_d_id);