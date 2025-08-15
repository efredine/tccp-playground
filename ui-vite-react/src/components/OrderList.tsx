import React from 'react';
import {
  Box,
  Card,
  CardContent,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  Chip,
  Pagination,
  CircularProgress,
  Alert,
  IconButton,
  Tooltip,
  TableSortLabel,
} from '@mui/material';
import {
  Visibility as ViewIcon,
  LocalShipping as DeliveredIcon,
  Schedule as PendingIcon,
} from '@mui/icons-material';
import type { OrderSummary, OrdersListResponse, OrdersQuery } from '../types/orders';

interface OrderListProps {
  data?: OrdersListResponse;
  isLoading?: boolean;
  error?: Error | null;
  query: OrdersQuery;
  onQueryChange: (newQuery: OrdersQuery) => void;
  onViewOrder?: (order: OrderSummary) => void;
}

export default function OrderList({ 
  data, 
  isLoading = false, 
  error, 
  query,
  onQueryChange,
  onViewOrder 
}: OrderListProps) {
  const handlePageChange = (_event: React.ChangeEvent<unknown>, value: number) => {
    onQueryChange({ ...query, page: value });
  };

  const handleSortChange = (sortField: OrdersQuery['sort_by']) => {
    const newSortDir = query.sort_by === sortField && query.sort_dir === 'asc' ? 'desc' : 'asc';
    onQueryChange({ 
      ...query, 
      sort_by: sortField, 
      sort_dir: newSortDir,
      page: 1 // Reset to first page when sorting changes
    });
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return 'N/A';
    try {
      return new Date(dateString).toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
      });
    } catch {
      return 'Invalid Date';
    }
  };

  const formatCurrency = (amount?: string) => {
    if (!amount) return 'N/A';
    try {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD',
      }).format(parseFloat(amount));
    } catch {
      return amount;
    }
  };

  const getDeliveryStatus = (order: OrderSummary) => {
    return order.is_delivered ? (
      <Chip 
        icon={<DeliveredIcon />}
        label="Delivered" 
        color="success" 
        size="small" 
      />
    ) : (
      <Chip 
        icon={<PendingIcon />}
        label="Pending" 
        color="warning" 
        size="small" 
      />
    );
  };

  const getCustomerName = (order: OrderSummary) => {
    const parts = [order.customer_first, order.customer_middle, order.customer_last]
      .filter(Boolean);
    return parts.length > 0 ? parts.join(' ') : `Customer ${order.o_c_id || 'N/A'}`;
  };

  if (error) {
    return (
      <Alert severity="error" sx={{ mt: 2 }}>
        <Typography variant="h6">Error loading orders</Typography>
        <Typography variant="body2">{error.message}</Typography>
      </Alert>
    );
  }

  return (
    <Card elevation={2} sx={{ mt: 2 }}>
      <CardContent>
        <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
          <Typography variant="h6" component="h2">
            Order Results
            {data && (
              <Typography variant="body2" color="text.secondary" component="span" ml={1}>
                ({data.total_count.toLocaleString()} total)
              </Typography>
            )}
          </Typography>
          
          {isLoading && <CircularProgress size={24} />}
        </Box>

        {isLoading && !data ? (
          <Box display="flex" justifyContent="center" p={4}>
            <CircularProgress />
          </Box>
        ) : !data || data.orders.length === 0 ? (
          <Box textAlign="center" py={4}>
            <Typography variant="body1" color="text.secondary">
              No orders found matching your search criteria.
            </Typography>
          </Box>
        ) : (
          <>
            <TableContainer>
              <Table size="small">
                <TableHead>
                  <TableRow>
                    <TableCell>
                      <TableSortLabel
                        active={query.sort_by === 'order_id'}
                        direction={query.sort_by === 'order_id' ? query.sort_dir : 'asc'}
                        onClick={() => handleSortChange('order_id')}
                      >
                        Order ID
                      </TableSortLabel>
                    </TableCell>
                    <TableCell>
                      <TableSortLabel
                        active={query.sort_by === 'customer_last'}
                        direction={query.sort_by === 'customer_last' ? query.sort_dir : 'asc'}
                        onClick={() => handleSortChange('customer_last')}
                      >
                        Customer
                      </TableSortLabel>
                    </TableCell>
                    <TableCell>
                      <TableSortLabel
                        active={query.sort_by === 'warehouse_id'}
                        direction={query.sort_by === 'warehouse_id' ? query.sort_dir : 'asc'}
                        onClick={() => handleSortChange('warehouse_id')}
                      >
                        W/D
                      </TableSortLabel>
                    </TableCell>
                    <TableCell>
                      <TableSortLabel
                        active={query.sort_by === 'entry_date'}
                        direction={query.sort_by === 'entry_date' ? query.sort_dir : 'asc'}
                        onClick={() => handleSortChange('entry_date')}
                      >
                        Entry Date
                      </TableSortLabel>
                    </TableCell>
                    <TableCell align="right">Total</TableCell>
                    <TableCell align="center">Lines</TableCell>
                    <TableCell align="center">Status</TableCell>
                    <TableCell align="center">
                      <TableSortLabel
                        active={query.sort_by === 'carrier_id'}
                        direction={query.sort_by === 'carrier_id' ? query.sort_dir : 'asc'}
                        onClick={() => handleSortChange('carrier_id')}
                      >
                        Carrier
                      </TableSortLabel>
                    </TableCell>
                    <TableCell align="center">Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {data.orders.map((order, index) => (
                    <TableRow 
                      key={`${order.o_w_id}-${order.o_d_id}-${order.o_id}-${index}`} 
                      hover 
                      sx={{ 
                        '&:last-child td, &:last-child th': { border: 0 },
                        cursor: onViewOrder ? 'pointer' : 'default',
                      }}
                      onClick={() => onViewOrder?.(order)}
                    >
                      <TableCell>
                        <Typography variant="body2" fontWeight="medium">
                          #{order.o_id}
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2">
                          {getCustomerName(order)}
                        </Typography>
                        <Typography variant="caption" color="text.secondary">
                          ID: {order.o_c_id || 'N/A'}
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2">
                          {order.o_w_id}/{order.o_d_id}
                        </Typography>
                      </TableCell>
                      <TableCell>
                        <Typography variant="body2">
                          {formatDate(order.o_entry_d)}
                        </Typography>
                      </TableCell>
                      <TableCell align="right">
                        <Typography variant="body2" fontWeight="medium">
                          {formatCurrency(order.total_amount)}
                        </Typography>
                      </TableCell>
                      <TableCell align="center">
                        <Chip 
                          label={order.line_count} 
                          size="small" 
                          variant="outlined"
                        />
                      </TableCell>
                      <TableCell align="center">
                        {getDeliveryStatus(order)}
                      </TableCell>
                      <TableCell align="center">
                        <Typography variant="body2">
                          {order.o_carrier_id || 'None'}
                        </Typography>
                      </TableCell>
                      <TableCell align="center">
                        <Tooltip title="View Order Details">
                          <IconButton 
                            size="small" 
                            onClick={(e) => {
                              e.stopPropagation();
                              onViewOrder?.(order);
                            }}
                          >
                            <ViewIcon />
                          </IconButton>
                        </Tooltip>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>

            {/* Pagination */}
            {data.total_pages > 1 && (
              <Box display="flex" justifyContent="center" mt={3}>
                <Pagination
                  count={data.total_pages}
                  page={data.page}
                  onChange={handlePageChange}
                  showFirstButton
                  showLastButton
                  disabled={isLoading}
                />
              </Box>
            )}

            {/* Summary */}
            <Box mt={2} pt={2} borderTop={1} borderColor="divider">
              <Typography variant="body2" color="text.secondary" textAlign="center">
                Showing {((data.page - 1) * data.per_page) + 1} to{' '}
                {Math.min(data.page * data.per_page, data.total_count)} of{' '}
                {data.total_count.toLocaleString()} orders
              </Typography>
            </Box>
          </>
        )}
      </CardContent>
    </Card>
  );
}
