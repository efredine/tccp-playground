import { createFileRoute } from '@tanstack/react-router';
import React, { useState } from 'react';
import { 
  Typography, 
  Box,
  Container,
} from '@mui/material';
import { useOrders } from '../hooks/useOrders';
import type { OrdersQuery, OrderSummary } from '../types/orders';
import OrderSearch from '../components/OrderSearch';
import OrderList from '../components/OrderList';
import OrderDetails from '../components/OrderDetails';

export const Route = createFileRoute('/orders')({
  component: OrdersPage,
});

function OrdersPage() {
  const [query, setQuery] = useState<OrdersQuery>({
    page: 1,
    per_page: 20,
    sort_by: 'entry_date',
    sort_dir: 'desc',
  });

  const [selectedOrder, setSelectedOrder] = useState<OrderSummary | null>(null);
  const [detailsOpen, setDetailsOpen] = useState(false);

  // Fetch orders based on current query
  const { data, isLoading, error } = useOrders(query);

  // Debug logging can be re-enabled if needed for troubleshooting

  const handleSearch = (newQuery: OrdersQuery) => {
    setQuery(newQuery);
  };

  const handleQueryChange = (newQuery: OrdersQuery) => {
    setQuery(newQuery);
  };

  const handleViewOrder = (order: OrderSummary) => {
    setSelectedOrder(order);
    setDetailsOpen(true);
  };

  const handleCloseDetails = () => {
    setDetailsOpen(false);
    setSelectedOrder(null);
  };

  return (
    <Container maxWidth="xl">
      <Box>
        <Typography variant="h4" component="h1" gutterBottom>
          Order Management
        </Typography>
        
        <Typography variant="body1" color="text.secondary" gutterBottom>
          Search, filter, and manage orders across all warehouses and districts.
        </Typography>
        
        {/* Search Component */}
        <OrderSearch 
          onSearch={handleSearch}
          isLoading={isLoading}
        />
        
        {/* Results List */}
        <OrderList
          data={data}
          isLoading={isLoading}
          error={error}
          query={query}
          onQueryChange={handleQueryChange}
          onViewOrder={handleViewOrder}
        />
        
        {/* Order Details Dialog */}
        <OrderDetails
          open={detailsOpen}
          order={selectedOrder}
          onClose={handleCloseDetails}
        />
      </Box>
    </Container>
  );
}
