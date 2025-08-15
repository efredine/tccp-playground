import { API_BASE_URL, API_ENDPOINTS } from '../config/api';
import type {
  OrdersQuery,
  OrdersListResponse,
  OrderStatusQuery,
  OrderStatusResponse,
} from '../types/orders';

// Re-export order management types from types file
export type {
  OrdersQuery,
  OrderSummary,
  OrdersListResponse,
  OrderStatusQuery,
  CustomerInfo,
  LatestOrderInfo,
  OrderLineInfo,
  OrderStatusResponse,
} from '../types/orders';

// ===== NEW ORDER TYPES =====

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

export interface CustomerSummary {
  customer_id: number;
  last_name: string;
  credit: string;
  discount: string; // BigDecimal as string
}

export interface OrderLineSummary {
  item_id: number;
  supply_warehouse_id: number;
  quantity: number;
  item_name: string;
  item_price: string; // BigDecimal as string
  stock_quantity: number;
  brand_generic: string;
  line_amount: string; // BigDecimal as string
}

export interface NewOrderResponse {
  order_id: number;
  customer: CustomerSummary;
  warehouse_tax: string; // BigDecimal as string
  district_tax: string; // BigDecimal as string
  order_entry_date: string; // ISO datetime string
  total_amount: string; // BigDecimal as string
  order_lines: OrderLineSummary[];
}

// ===== NEW ORDER API =====

export const submitNewOrder = async (orderRequest: NewOrderRequest): Promise<NewOrderResponse> => {
  const response = await fetch(`${API_BASE_URL}${API_ENDPOINTS.newOrder}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(orderRequest),
  });

  if (!response.ok) {
    const errorText = await response.text().catch(() => 'Unknown error');
    throw new Error(`Failed to submit order: ${response.status} ${response.statusText}. ${errorText}`);
  }

  const orderResponse: NewOrderResponse = await response.json();
  return orderResponse;
};

// ===== ORDER MANAGEMENT API =====

export const fetchOrders = async (query: OrdersQuery = {}): Promise<OrdersListResponse> => {
  const params = new URLSearchParams();
  
  // Add query parameters
  if (query.warehouse_id !== undefined) params.append('warehouse_id', query.warehouse_id.toString());
  if (query.district_id !== undefined) params.append('district_id', query.district_id.toString());
  if (query.customer_id !== undefined) params.append('customer_id', query.customer_id.toString());
  if (query.order_id !== undefined) params.append('order_id', query.order_id.toString());
  if (query.from_date) params.append('from_date', query.from_date);
  if (query.to_date) params.append('to_date', query.to_date);
  if (query.page !== undefined) params.append('page', query.page.toString());
  if (query.per_page !== undefined) params.append('per_page', query.per_page.toString());
  if (query.sort_by) params.append('sort_by', query.sort_by);
  if (query.sort_dir) params.append('sort_dir', query.sort_dir);
  
  const url = `${API_BASE_URL}${API_ENDPOINTS.orders}${params.toString() ? '?' + params.toString() : ''}`;
  
  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    const errorText = await response.text().catch(() => 'Unknown error');
    throw new Error(`Failed to fetch orders: ${response.status} ${response.statusText}. ${errorText}`);
  }

  const ordersResponse: OrdersListResponse = await response.json();
  return ordersResponse;
};

// ===== ORDER STATUS API =====

export const fetchOrderStatus = async (query: OrderStatusQuery): Promise<OrderStatusResponse> => {
  const params = new URLSearchParams({
    warehouse_id: query.warehouse_id.toString(),
    district_id: query.district_id.toString(),
    customer_id: query.customer_id.toString(),
  });
  
  const url = `${API_BASE_URL}${API_ENDPOINTS.orderStatus}?${params.toString()}`;
  
  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    const errorText = await response.text().catch(() => 'Unknown error');
    throw new Error(`Failed to fetch order status: ${response.status} ${response.statusText}. ${errorText}`);
  }

  const orderStatusResponse: OrderStatusResponse = await response.json();
  return orderStatusResponse;
};
