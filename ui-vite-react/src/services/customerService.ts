import { API_BASE_URL, API_ENDPOINTS } from '../config/api';
import type { Customer } from '../types/order.types';

export const searchCustomers = async (
  warehouseId: number,
  districtId: number,
  searchQuery: string = '',
  limit: number = 20
): Promise<Customer[]> => {
  const params = new URLSearchParams();
  params.append('warehouse_id', warehouseId.toString());
  params.append('district_id', districtId.toString());
  if (searchQuery.trim()) {
    params.append('search', searchQuery.trim());
  }
  params.append('limit', limit.toString());

  const url = `${API_BASE_URL}${API_ENDPOINTS.customers}?${params.toString()}`;
  const response = await fetch(url);
  
  if (!response.ok) {
    throw new Error(`Failed to search customers: ${response.status} ${response.statusText}`);
  }
  
  const customers: Customer[] = await response.json();
  return customers;
};