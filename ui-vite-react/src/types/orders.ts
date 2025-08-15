// Order Management Types

export interface OrdersQuery {
  // Filtering criteria
  warehouse_id?: number;
  district_id?: number;
  customer_id?: number;
  order_id?: number;
  
  // Date range filtering
  from_date?: string; // ISO date string
  to_date?: string;   // ISO date string
  
  // Pagination
  page?: number;
  per_page?: number;
  
  // Sorting
  sort_by?: 'order_id' | 'entry_date' | 'customer_last' | 'warehouse_id' | 'district_id' | 'carrier_id';
  sort_dir?: 'asc' | 'desc';
}

export interface OrderSummary {
  o_id: number;
  o_w_id: number;
  o_d_id: number;
  o_c_id?: number;
  o_entry_d?: string; // ISO datetime string
  o_carrier_id?: number;
  o_ol_cnt?: number;
  o_all_local?: number;
  
  // Customer info for display
  customer_first?: string;
  customer_middle?: string;
  customer_last?: string;
  
  // Order total (calculated from order lines)
  total_amount?: string; // BigDecimal as string
  
  // Status indicators
  is_delivered: boolean;
  line_count: number;
}

export interface OrdersListResponse {
  orders: OrderSummary[];
  total_count: number;
  page: number;
  per_page: number;
  total_pages: number;
}

// Order Status Types (for TPC-C Order Status transaction)
export interface OrderStatusQuery {
  warehouse_id: number;
  district_id: number;
  customer_id: number;
}

export interface CustomerInfo {
  c_id: number;
  c_first?: string;
  c_middle?: string;
  c_last?: string;
  c_balance?: string; // BigDecimal as string
}

export interface LatestOrderInfo {
  o_id: number;
  o_entry_d?: string; // ISO datetime string
  o_carrier_id?: number;
}

export interface OrderLineInfo {
  ol_i_id?: number;
  ol_supply_w_id?: number;
  ol_quantity?: number;
  ol_amount?: string; // BigDecimal as string
  ol_delivery_d?: string; // ISO datetime string
}

export interface OrderStatusResponse {
  customer: CustomerInfo;
  latest_order: LatestOrderInfo;
  order_lines: OrderLineInfo[];
}