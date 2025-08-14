import { API_BASE_URL, API_ENDPOINTS } from '../config/api';
import type { Warehouse } from '../types/order.types';

export const fetchWarehouses = async (): Promise<Warehouse[]> => {
  const response = await fetch(`${API_BASE_URL}${API_ENDPOINTS.warehouses}`);
  
  if (!response.ok) {
    throw new Error(`Failed to fetch warehouses: ${response.status} ${response.statusText}`);
  }
  
  const warehouses: Warehouse[] = await response.json();
  return warehouses;
};