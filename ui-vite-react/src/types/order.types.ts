// TPC-C Order Management Types

export interface Warehouse {
  w_id: number;
  w_name: string;
  w_street_1?: string;
  w_street_2?: string;
  w_city?: string;
  w_state?: string;
  w_zip?: string;
  w_tax?: string;
}

export interface District {
  d_id: number;
  d_w_id: number;
  d_name: string;
  d_street_1?: string;
  d_street_2?: string;
  d_city?: string;
  d_state?: string;
  d_zip?: string;
  d_tax?: string;
}

export interface Customer {
  c_id: number;
  c_d_id: number;
  c_w_id: number;
  c_first: string;
  c_middle: string;
  c_last: string;
  c_street_1?: string;
  c_street_2?: string;
  c_city?: string;
  c_state?: string;
  c_zip?: string;
  c_phone?: string;
  c_since?: string;
  c_credit?: string;
  c_credit_lim?: number;
  c_discount?: string;
  c_balance?: string;
}

export interface Item {
  i_id: number;
  i_name: string;
  i_price: string;
  i_data?: string;
}

export interface OrderLineRequest {
  item_id: number;
  supply_warehouse_id: number;
  quantity: number;
}

export interface NewOrderRequest {
  warehouse_id: number;
  district_id: number;
  customer_id: number;
  order_lines: OrderLineRequest[];
}

export interface OrderLineSummary {
  item_id: number;
  supply_warehouse_id: number;
  quantity: number;
  item_name: string;
  item_price: string;
  stock_quantity: number;
  brand_generic: string;
  line_amount: string;
}

export interface CustomerSummary {
  customer_id: number;
  last_name: string;
  credit: string;
  discount: string;
}

export interface NewOrderResponse {
  order_id: number;
  customer: CustomerSummary;
  warehouse_tax: string;
  district_tax: string;
  order_entry_date: string;
  total_amount: string;
  order_lines: OrderLineSummary[];
}

export interface Order {
  o_id: number;
  o_d_id: number;
  o_w_id: number;
  o_c_id: number;
  o_entry_d: string;
  o_carrier_id?: number;
  o_ol_cnt: number;
  o_all_local: number;
  status: 'New' | 'Delivered';
  customer_name?: string;
  total_amount?: string;
}

export interface OrdersListParams {
  warehouse_id?: number;
  district_id?: number;
  customer_id?: number;
  status?: 'new' | 'delivered' | 'all';
  page?: number;
  limit?: number;
}