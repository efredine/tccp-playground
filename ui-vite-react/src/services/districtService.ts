import { API_BASE_URL, API_ENDPOINTS } from '../config/api';
import type { District } from '../types/order.types';

export const fetchDistricts = async (warehouseId: number): Promise<District[]> => {
  const response = await fetch(`${API_BASE_URL}${API_ENDPOINTS.districts}?warehouse_id=${warehouseId}`);
  
  if (!response.ok) {
    throw new Error(`Failed to fetch districts: ${response.status} ${response.statusText}`);
  }
  
  const districts: District[] = await response.json();
  return districts;
};