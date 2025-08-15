import { useQuery } from '@tanstack/react-query';
import { searchCustomers } from '../services/customerService';
import { useDebounce } from './useDebounce';

export const useCustomers = (
  warehouseId?: number,
  districtId?: number,
  searchQuery?: string
) => {
  const debouncedSearchQuery = useDebounce(searchQuery || '', 300); // Debounce search by 300ms
  
  // Track if we're waiting for debounce
  const isDebouncing = searchQuery !== debouncedSearchQuery;

  const query = useQuery({
    queryKey: ['customers', warehouseId, districtId, debouncedSearchQuery],
    queryFn: () => searchCustomers(warehouseId!, districtId!, debouncedSearchQuery),
    enabled: !!warehouseId && !!districtId, // Only run if W/D selected
    staleTime: 1000 * 60, // 1 minute stale time for search results
    retry: 1,
  });

  return {
    ...query,
    isDebouncing,
    // Combined loading state: true if debouncing OR fetching
    isSearching: isDebouncing || query.isFetching,
  };
};
