// API Configuration

export const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

export const API_ENDPOINTS = {
  // Existing endpoints
  warehouses: '/warehouses',
  newOrder: '/new-order',
  orderStatus: '/order-status',
  payment: '/payment',
  delivery: '/delivery',
  stockLevel: '/stock-level',
  
  // New endpoints needed for UI
  items: '/items',
  customers: '/customers', 
  orders: '/orders',
  districts: '/districts',
} as const;

// API client configuration
export const apiClient = {
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
};