import { useQuery } from '@tanstack/react-query';
import { fetchOrders } from '../services/orderService';
import type { OrdersQuery, OrdersListResponse } from '../types/orders';

// Hook for fetching orders with search and pagination
export const useOrders = (query: OrdersQuery = {}) => {
  return useQuery<OrdersListResponse, Error>({
    queryKey: ['orders', query],
    queryFn: () => fetchOrders(query),
    staleTime: 5 * 60 * 1000, // 5 minutes
    gcTime: 10 * 60 * 1000, // 10 minutes
  });
};

// Hook for fetching orders with refetch capability
export const useOrdersWithRefresh = (query: OrdersQuery = {}) => {
  const result = useQuery<OrdersListResponse, Error>({
    queryKey: ['orders', query],
    queryFn: () => fetchOrders(query),
    staleTime: 5 * 60 * 1000, // 5 minutes
    gcTime: 10 * 60 * 1000, // 10 minutes
  });

  return {
    ...result,
    refetchOrders: result.refetch,
  };
};
