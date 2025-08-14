import { useQuery } from '@tanstack/react-query';
import { fetchDistricts } from '../services/districtService';

export const useDistricts = (warehouseId?: number) => {
  return useQuery({
    queryKey: ['districts', warehouseId],
    queryFn: () => fetchDistricts(warehouseId!),
    enabled: !!warehouseId, // Only run query when warehouseId is provided
    staleTime: 1000 * 60 * 5, // 5 minutes - districts don't change often
    retry: 2,
  });
};