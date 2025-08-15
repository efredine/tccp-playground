import {
  Paper,
  Typography,
  Box,
  Grid,
  Divider,
  Chip,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Alert,
  Button,
} from '@mui/material';
import { CheckCircle, Add } from '@mui/icons-material';
import type { NewOrderResponse } from '../services/orderService';

interface OrderConfirmationProps {
  order: NewOrderResponse;
  onCreateAnother?: () => void;
}

export function OrderConfirmation({ order, onCreateAnother }: OrderConfirmationProps) {
  const formatCurrency = (amount: string) => {
    return `$${parseFloat(amount).toFixed(2)}`;
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  return (
    <Paper sx={{ p: 4 }}>
      {/* Success Header */}
      <Box sx={{ textAlign: 'center', mb: 4 }}>
        <CheckCircle sx={{ fontSize: 64, color: 'success.main', mb: 2 }} />
        <Typography variant="h4" color="success.main" gutterBottom>
          Order Submitted Successfully!
        </Typography>
        <Typography variant="h6" color="text.secondary">
          Order ID: {order.order_id}
        </Typography>
        <Typography variant="body2" color="text.secondary">
          Submitted: {formatDate(order.order_entry_date)}
        </Typography>
      </Box>

      <Divider sx={{ mb: 4 }} />

      {/* Order Summary */}
      <Grid container spacing={4} sx={{ mb: 4 }}>
        <Grid size={{ xs: 12, md: 6 }}>
          <Typography variant="h6" gutterBottom>
            Customer Information
          </Typography>
          <Typography variant="body1">
            <strong>{order.customer.last_name}</strong> (ID: {order.customer.customer_id})
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Credit Status: {order.customer.credit}
          </Typography>
          <Typography variant="body2" color="text.secondary">
            Discount: {(parseFloat(order.customer.discount) * 100).toFixed(1)}%
          </Typography>
        </Grid>

        <Grid size={{ xs: 12, md: 6 }}>
          <Typography variant="h6" gutterBottom>
            Order Totals
          </Typography>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
            <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
              <Typography variant="body2">Subtotal:</Typography>
              <Typography variant="body2">
                {formatCurrency(
                  order.order_lines.reduce((sum, line) => sum + parseFloat(line.line_amount), 0).toFixed(2)
                )}
              </Typography>
            </Box>
            <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
              <Typography variant="body2">Warehouse Tax:</Typography>
              <Typography variant="body2">{(parseFloat(order.warehouse_tax) * 100).toFixed(2)}%</Typography>
            </Box>
            <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
              <Typography variant="body2">District Tax:</Typography>
              <Typography variant="body2">{(parseFloat(order.district_tax) * 100).toFixed(2)}%</Typography>
            </Box>
            <Divider />
            <Box sx={{ display: 'flex', justifyContent: 'space-between' }}>
              <Typography variant="h6">Total Amount:</Typography>
              <Typography variant="h6" color="primary">
                {formatCurrency(order.total_amount)}
              </Typography>
            </Box>
          </Box>
        </Grid>
      </Grid>

      {/* Order Lines */}
      <Typography variant="h6" gutterBottom sx={{ mt: 4 }}>
        Order Items ({order.order_lines.length} items)
      </Typography>
      
      <TableContainer component={Paper} variant="outlined" sx={{ mt: 2 }}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell><strong>Item</strong></TableCell>
              <TableCell align="center"><strong>Supply WH</strong></TableCell>
              <TableCell align="center"><strong>Quantity</strong></TableCell>
              <TableCell align="right"><strong>Unit Price</strong></TableCell>
              <TableCell align="center"><strong>Stock After</strong></TableCell>
              <TableCell align="center"><strong>Type</strong></TableCell>
              <TableCell align="right"><strong>Line Total</strong></TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {order.order_lines.map((line, index) => (
              <TableRow key={index}>
                <TableCell>
                  <Typography variant="body2" fontWeight="medium">
                    {line.item_name}
                  </Typography>
                  <Typography variant="caption" color="text.secondary">
                    ID: {line.item_id}
                  </Typography>
                </TableCell>
                <TableCell align="center">
                  <Chip 
                    label={`WH ${line.supply_warehouse_id}`} 
                    size="small" 
                    variant="outlined" 
                  />
                </TableCell>
                <TableCell align="center">{line.quantity}</TableCell>
                <TableCell align="right">{formatCurrency(line.item_price)}</TableCell>
                <TableCell align="center">
                  <Typography 
                    variant="body2" 
                    color={line.stock_quantity < 10 ? 'warning.main' : 'text.primary'}
                  >
                    {line.stock_quantity}
                  </Typography>
                </TableCell>
                <TableCell align="center">
                  <Chip 
                    label={line.brand_generic === 'B' ? 'Brand' : 'Generic'} 
                    size="small"
                    color={line.brand_generic === 'B' ? 'primary' : 'default'}
                  />
                </TableCell>
                <TableCell align="right" sx={{ fontWeight: 'medium' }}>
                  {formatCurrency(line.line_amount)}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>

      {/* Low Stock Warning */}
      {order.order_lines.some(line => line.stock_quantity < 10) && (
        <Alert severity="warning" sx={{ mt: 3 }}>
          <Typography variant="subtitle2" gutterBottom>
            Low Stock Alert
          </Typography>
          Some items now have low stock levels (less than 10 units remaining).
          Consider restocking soon.
        </Alert>
      )}

      {/* Remote Orders Info */}
      {order.order_lines.some(line => line.supply_warehouse_id !== order.order_lines[0]?.supply_warehouse_id) && (
        <Alert severity="info" sx={{ mt: 2 }}>
          <Typography variant="subtitle2" gutterBottom>
            Multi-Warehouse Order
          </Typography>
          This order includes items from multiple warehouses, which may affect delivery times.
        </Alert>
      )}

      {/* Action Buttons */}
      <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}>
        <Button
          variant="contained"
          size="large"
          startIcon={<Add />}
          onClick={onCreateAnother}
        >
          Create Another Order
        </Button>
      </Box>
    </Paper>
  );
}