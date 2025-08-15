// API Configuration

// Auto-detect API base URL based on environment
const getApiBaseUrl = () => {
  // If explicitly set in environment, use that
  if (import.meta.env.VITE_API_BASE_URL) {
    return import.meta.env.VITE_API_BASE_URL;
  }
  
  // Check if we're running in a browser environment
  if (typeof window !== 'undefined') {
    const port = window.location.port;
    
    // If served from Vite dev server ports, we're in development
    if (port === '5173' || port === '3000') {
      return 'http://localhost:8080';
    }
    
    // If served from port 8080 or no port (production), use relative /api
    // This covers localhost:8080, 127.0.0.1:8080, and any other hostname:8080
    if (port === '8080' || port === '' || port === '80' || port === '443') {
      return '/api';
    }
  }
  
  // Fallback: use Vite's mode detection
  if (import.meta.env.PROD) {
    return '/api';
  }
  
  // Final fallback for development
  return 'http://localhost:8080';
};

export const API_BASE_URL = getApiBaseUrl();

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