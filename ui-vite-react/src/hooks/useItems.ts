import { useQuery } from '@tanstack/react-query';
import { searchItems, getStockInfo } from '../services/itemService';
import { useDebounce } from './useDebounce';

export const useItems = (
  warehouseId?: number,
  searchQuery?: string
) => {
  const debouncedSearchQuery = useDebounce(searchQuery || '', 300); // Debounce search by 300ms
  
  // Track if we're waiting for debounce
  const isDebouncing = searchQuery !== debouncedSearchQuery;

  const query = useQuery({
    queryKey: ['items', warehouseId, debouncedSearchQuery],
    queryFn: () => searchItems(warehouseId!, debouncedSearchQuery),
    enabled: !!warehouseId, // Only run if warehouse is selected
    staleTime: 1000 * 60 * 5, // 5 minute stale time for item search results
    retry: 1,
  });

  return {
    ...query,
    isDebouncing,
    // Combined loading state: true if debouncing OR fetching
    isSearching: isDebouncing || query.isFetching,
  };
};

export const useStockInfo = (warehouseId?: number, itemId?: number) => {
  return useQuery({
    queryKey: ['stock', warehouseId, itemId],
    queryFn: () => getStockInfo(warehouseId!, itemId!),
    enabled: !!warehouseId && !!itemId,
    staleTime: 1000 * 30, // 30 seconds stale time for stock info
    retry: 1,
  });
};