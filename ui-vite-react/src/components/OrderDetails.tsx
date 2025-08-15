import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  Box,
  Typography,
  Grid,
  Card,
  CardContent,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Chip,
  CircularProgress,
  Alert,
  IconButton,
} from '@mui/material';
import {
  Close as CloseIcon,
  LocalShipping as DeliveredIcon,
  Schedule as PendingIcon,
  Person as PersonIcon,
  Business as WarehouseIcon,
  Receipt as OrderIcon,
} from '@mui/icons-material';
import type { OrderSummary } from '../types/orders';
import { useOrderStatus } from '../hooks/useOrderStatus';

interface OrderDetailsProps {
  open: boolean;
  order: OrderSummary | null;
  onClose: () => void;
}

export default function OrderDetails({ open, order, onClose }: OrderDetailsProps) {
  // Fetch detailed order status when order is selected
  const { data: orderStatus, isLoading, error } = useOrderStatus(
    order ? {
      warehouse_id: order.o_w_id,
      district_id: order.o_d_id,
      customer_id: order.o_c_id || 0,
    } : {} as any,
    open && !!order && !!order.o_c_id
  );

  const formatDate = (dateString?: string) => {
    if (!dateString) return 'N/A';
    try {
      return new Date(dateString).toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
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

  const getCustomerName = () => {
    if (orderStatus) {
      const parts = [
        orderStatus.customer.c_first,
        orderStatus.customer.c_middle,
        orderStatus.customer.c_last
      ].filter(Boolean);
      return parts.join(' ') || `Customer ${orderStatus.customer.c_id}`;
    }
    if (order) {
      const parts = [order.customer_first, order.customer_middle, order.customer_last]
        .filter(Boolean);
      return parts.join(' ') || `Customer ${order.o_c_id || 'N/A'}`;
    }
    return 'Unknown Customer';
  };

  const getDeliveryStatus = () => {
    const isDelivered = order?.is_delivered || !!orderStatus?.latest_order.o_carrier_id;
    return isDelivered ? (
      <Chip 
        icon={<DeliveredIcon />}
        label="Delivered" 
        color="success" 
        size="small" 
      />
    ) : (
      <Chip 
        icon={<PendingIcon />}
        label="Pending Delivery" 
        color="warning" 
        size="small" 
      />
    );
  };

  if (!order) return null;

  return (
    <Dialog 
      open={open} 
      onClose={onClose} 
      maxWidth="md" 
      fullWidth
      PaperProps={{ elevation: 8 }}
    >
      <DialogTitle>
        <Box display="flex" alignItems="center" justifyContent="space-between">
          <Box display="flex" alignItems="center" gap={1}>
            <OrderIcon color="primary" />
            <Typography variant="h5" component="h2">
              Order #{order.o_id}
            </Typography>
            {getDeliveryStatus()}
          </Box>
          <IconButton onClick={onClose} size="small">
            <CloseIcon />
          </IconButton>
        </Box>
      </DialogTitle>

      <DialogContent>
        {error && (
          <Alert severity="error" sx={{ mb: 2 }}>
            <Typography variant="body2">
              Failed to load detailed order information: {error.message}
            </Typography>
          </Alert>
        )}

        {/* Order Overview */}
        <Card elevation={2} sx={{ mb: 2 }}>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              <WarehouseIcon sx={{ mr: 1, verticalAlign: 'middle' }} />
              Order Information
            </Typography>
            
            <Grid container spacing={2}>
              <Grid size={{ xs: 12, sm: 6 }}>
                <Typography variant="body2" color="text.secondary">Order ID</Typography>
                <Typography variant="body1" fontWeight="medium">#{order.o_id}</Typography>
              </Grid>
              <Grid size={{ xs: 12, sm: 6 }}>
                <Typography variant="body2" color="text.secondary">Entry Date</Typography>
                <Typography variant="body1">{formatDate(order.o_entry_d)}</Typography>
              </Grid>
              <Grid size={{ xs: 12, sm: 6 }}>
                <Typography variant="body2" color="text.secondary">Warehouse / District</Typography>
                <Typography variant="body1">{order.o_w_id} / {order.o_d_id}</Typography>
              </Grid>
              <Grid size={{ xs: 12, sm: 6 }}>
                <Typography variant="body2" color="text.secondary">Carrier ID</Typography>
                <Typography variant="body1">
                  {orderStatus?.latest_order.o_carrier_id || order.o_carrier_id || 'Not assigned'}
                </Typography>
              </Grid>
              <Grid size={{ xs: 12, sm: 6 }}>
                <Typography variant="body2" color="text.secondary">Total Amount</Typography>
                <Typography variant="h6" color="primary">
                  {formatCurrency(order.total_amount)}
                </Typography>
              </Grid>
              <Grid size={{ xs: 12, sm: 6 }}>
                <Typography variant="body2" color="text.secondary">Line Count</Typography>
                <Typography variant="body1">
                  {orderStatus?.order_lines.length || order.line_count} items
                </Typography>
              </Grid>
            </Grid>
          </CardContent>
        </Card>

        {/* Customer Information */}
        <Card elevation={2} sx={{ mb: 2 }}>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              <PersonIcon sx={{ mr: 1, verticalAlign: 'middle' }} />
              Customer Information
            </Typography>
            
            {isLoading ? (
              <Box display="flex" justifyContent="center" p={2}>
                <CircularProgress size={20} />
              </Box>
            ) : orderStatus ? (
              <Grid container spacing={2}>
                <Grid size={{ xs: 12, sm: 6 }}>
                  <Typography variant="body2" color="text.secondary">Customer ID</Typography>
                  <Typography variant="body1">{orderStatus.customer.c_id}</Typography>
                </Grid>
                <Grid size={{ xs: 12, sm: 6 }}>
                  <Typography variant="body2" color="text.secondary">Full Name</Typography>
                  <Typography variant="body1">{getCustomerName()}</Typography>
                </Grid>
                <Grid size={{ xs: 12, sm: 6 }}>
                  <Typography variant="body2" color="text.secondary">Balance</Typography>
                  <Typography variant="body1">
                    {formatCurrency(orderStatus.customer.c_balance)}
                  </Typography>
                </Grid>
              </Grid>
            ) : (
              <Typography variant="body2" color="text.secondary">
                {getCustomerName()} (ID: {order.o_c_id})
              </Typography>
            )}
          </CardContent>
        </Card>

        {/* Order Lines */}
        <Card elevation={2}>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              Order Line Items
            </Typography>
            
            {isLoading ? (
              <Box display="flex" justifyContent="center" p={4}>
                <CircularProgress />
              </Box>
            ) : orderStatus && orderStatus.order_lines.length > 0 ? (
              <TableContainer>
                <Table size="small">
                  <TableHead>
                    <TableRow>
                      <TableCell>Item ID</TableCell>
                      <TableCell>Supply Warehouse</TableCell>
                      <TableCell align="right">Quantity</TableCell>
                      <TableCell align="right">Amount</TableCell>
                      <TableCell>Delivery Date</TableCell>
                    </TableRow>
                  </TableHead>
                  <TableBody>
                    {orderStatus.order_lines.map((line, index) => (
                      <TableRow key={index}>
                        <TableCell>
                          <Typography variant="body2">
                            {line.ol_i_id || 'N/A'}
                          </Typography>
                        </TableCell>
                        <TableCell>
                          <Typography variant="body2">
                            {line.ol_supply_w_id || 'N/A'}
                          </Typography>
                        </TableCell>
                        <TableCell align="right">
                          <Typography variant="body2">
                            {line.ol_quantity || 'N/A'}
                          </Typography>
                        </TableCell>
                        <TableCell align="right">
                          <Typography variant="body2" fontWeight="medium">
                            {formatCurrency(line.ol_amount)}
                          </Typography>
                        </TableCell>
                        <TableCell>
                          <Typography variant="body2">
                            {line.ol_delivery_d ? formatDate(line.ol_delivery_d) : 'Not delivered'}
                          </Typography>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </TableContainer>
            ) : (
              <Typography variant="body2" color="text.secondary" textAlign="center" py={2}>
                {error ? 'Unable to load order line details' : 'No order line details available'}
              </Typography>
            )}
          </CardContent>
        </Card>
      </DialogContent>

      <DialogActions>
        <Button onClick={onClose} variant="outlined">
          Close
        </Button>
      </DialogActions>
    </Dialog>
  );
}
