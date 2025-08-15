import { useQuery } from '@tanstack/react-query';
import { fetchOrderStatus } from '../services/orderService';
import type { OrderStatusQuery, OrderStatusResponse } from '../types/orders';

// Hook for fetching order status (TPC-C Order Status transaction)
export const useOrderStatus = (query: OrderStatusQuery, enabled: boolean = true) => {
  return useQuery<OrderStatusResponse, Error>({
    queryKey: ['orderStatus', query],
    queryFn: () => fetchOrderStatus(query),
    enabled: enabled && !!query.warehouse_id && !!query.district_id && !!query.customer_id,
    staleTime: 30 * 1000, // 30 seconds
    gcTime: 2 * 60 * 1000, // 2 minutes
  });
};
