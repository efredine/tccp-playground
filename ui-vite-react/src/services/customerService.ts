import { API_BASE_URL, API_ENDPOINTS } from '../config/api';
import type { Customer } from '../types/order.types';

export const searchCustomers = async (
  warehouseId: number,
  districtId: number,
  searchQuery: string = '',
  limit: number = 20
): Promise<Customer[]> => {
  const url = new URL(`${API_BASE_URL}${API_ENDPOINTS.customers}`);
  url.searchParams.append('warehouse_id', warehouseId.toString());
  url.searchParams.append('district_id', districtId.toString());
  if (searchQuery.trim()) {
    url.searchParams.append('search', searchQuery.trim());
  }
  url.searchParams.append('limit', limit.toString());

  const response = await fetch(url.toString());
  
  if (!response.ok) {
    throw new Error(`Failed to search customers: ${response.status} ${response.statusText}`);
  }
  
  const customers: Customer[] = await response.json();
  return customers;
};