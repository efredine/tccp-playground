import { API_BASE_URL, API_ENDPOINTS } from '../config/api';

export interface Item {
  i_id: number;
  i_im_id?: number | null;
  i_name?: string | null;
  i_price?: string | null; // BigDecimal comes as string from API
  i_data?: string | null;
}

export interface StockInfo {
  s_quantity: number;
  s_ytd?: string | null; // BigDecimal comes as string from API  
  s_order_cnt: number;
  s_remote_cnt: number;
  s_data?: string | null;
}

export const searchItems = async (
  warehouseId: number,
  searchQuery: string = '',
  limit: number = 20
): Promise<Item[]> => {
  const params = new URLSearchParams();
  params.append('warehouse_id', warehouseId.toString());
  if (searchQuery.trim()) {
    params.append('search', searchQuery.trim());
  }
  params.append('limit', limit.toString());

  const url = `${API_BASE_URL}${API_ENDPOINTS.items}?${params.toString()}`;
  const response = await fetch(url);
  
  if (!response.ok) {
    throw new Error(`Failed to search items: ${response.status} ${response.statusText}`);
  }
  
  const items: Item[] = await response.json();
  return items;
};

export const getStockInfo = async (
  warehouseId: number,
  itemId: number
): Promise<StockInfo> => {
  const params = new URLSearchParams();
  params.append('warehouse_id', warehouseId.toString());
  params.append('item_id', itemId.toString());

  const url = `${API_BASE_URL}/stock?${params.toString()}`;
  const response = await fetch(url);
  
  if (!response.ok) {
    if (response.status === 404) {
      throw new Error('Stock information not found for this item');
    }
    throw new Error(`Failed to get stock info: ${response.status} ${response.statusText}`);
  }
  
  const stock: StockInfo = await response.json();
  return stock;
};