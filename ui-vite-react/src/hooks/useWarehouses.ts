import { useQuery } from '@tanstack/react-query';
import { fetchWarehouses } from '../services/warehouseService';

export const useWarehouses = () => {
  return useQuery({
    queryKey: ['warehouses'],
    queryFn: fetchWarehouses,
    staleTime: 1000 * 60 * 5, // 5 minutes - warehouses don't change often
    retry: 2,
  });
};