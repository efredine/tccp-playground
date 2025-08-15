import { API_BASE_URL, API_ENDPOINTS } from '../config/api';

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