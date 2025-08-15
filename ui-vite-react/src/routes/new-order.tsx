import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { 
  Typography, 
  Paper, 
  Box,
  Grid,
  Button,
  Alert,
  CircularProgress,
  Divider
} from '@mui/material';
import { CheckCircle as SubmitIcon } from '@mui/icons-material';
import { WarehouseSelect } from '../components/WarehouseSelect';
import { DistrictSelect } from '../components/DistrictSelect';
import { CustomerAutocomplete } from '../components/CustomerAutocomplete';
import { OrderLines, type OrderLine } from '../components/OrderLines';
import { useOrderSubmission } from '../hooks/useOrderSubmission';
import { validateOrder, formatValidationErrors } from '../utils/orderValidation';
import { OrderConfirmation } from '../components/OrderConfirmation';
import type { Customer } from '../types/order.types';

export const Route = createFileRoute('/new-order')({
  component: NewOrderPage,
});

function NewOrderPage() {
  const [selectedWarehouse, setSelectedWarehouse] = useState<number>();
  const [selectedDistrict, setSelectedDistrict] = useState<number>();
  const [selectedCustomer, setSelectedCustomer] = useState<Customer | null>(null);
  const [orderLines, setOrderLines] = useState<OrderLine[]>([
    {
      id: 'initial-line',
      item: null,
      quantity: 1,
    }
  ]);
  const [validationErrors, setValidationErrors] = useState<string[]>([]);
  
  const orderMutation = useOrderSubmission();

  // Reset district, customer, and order lines when warehouse changes
  const handleWarehouseChange = (warehouseId: number) => {
    setSelectedWarehouse(warehouseId);
    setSelectedDistrict(undefined); // Clear district selection
    setSelectedCustomer(null); // Clear customer selection
    // Reset order lines to initial state
    setOrderLines([{
      id: `line-${Date.now()}`,
      item: null,
      quantity: 1,
      supply_w_id: warehouseId,
    }]);
  };

  // Reset customer when district changes
  const handleDistrictChange = (districtId: number) => {
    setSelectedDistrict(districtId);
    setSelectedCustomer(null); // Clear customer selection
  };

  // Handle order submission
  const handleSubmitOrder = async () => {
    // Clear previous validation errors
    setValidationErrors([]);

    // Validate the order
    const validation = validateOrder(selectedWarehouse, selectedDistrict, selectedCustomer, orderLines);
    
    if (!validation.isValid) {
      setValidationErrors(validation.errors.map(err => err.message));
      return;
    }

    // Prepare order request - only include lines with items
    const validOrderLines = orderLines.filter(line => line.item);
    
    const orderRequest = {
      warehouse_id: selectedWarehouse!,
      district_id: selectedDistrict!,
      customer_id: selectedCustomer!.c_id,
      order_lines: validOrderLines.map(line => ({
        item_id: line.item!.i_id,
        supply_warehouse_id: line.supply_w_id || selectedWarehouse!,
        quantity: line.quantity,
      })),
    };

    try {
      const result = await orderMutation.mutateAsync(orderRequest);
      console.log('Order submitted successfully:', result);
      // Success state will be handled by the UI below
    } catch (error) {
      console.error('Failed to submit order:', error);
      setValidationErrors([error instanceof Error ? error.message : 'Failed to submit order']);
    }
  };

  // Check if form is ready for submission
  const canSubmitOrder = selectedWarehouse && selectedDistrict && selectedCustomer && 
    orderLines.some(line => line.item) && !orderMutation.isPending;

  // Calculate totals for summary
  const validLines = orderLines.filter(line => line.item);
  const totalOrderValue = validLines.reduce((sum, line) => {
    if (line.item?.i_price && line.quantity > 0) {
      return sum + (parseFloat(line.item.i_price) * line.quantity);
    }
    return sum;
  }, 0);

  // Handle creating another order (reset form)
  const handleCreateAnotherOrder = () => {
    // Reset all form state
    setSelectedWarehouse(undefined);
    setSelectedDistrict(undefined);
    setSelectedCustomer(null);
    setOrderLines([{
      id: `line-${Date.now()}`,
      item: null,
      quantity: 1,
    }]);
    setValidationErrors([]);
    orderMutation.reset();
  };

  // Show order confirmation if submission was successful
  if (orderMutation.isSuccess && orderMutation.data) {
    return (
      <Box>
        <Typography variant="h4" component="h1" gutterBottom>
          Order Confirmation
        </Typography>
        <OrderConfirmation 
          order={orderMutation.data} 
          onCreateAnother={handleCreateAnotherOrder}
        />
      </Box>
    );
  }

  return (
    <Box>
      <Typography variant="h4" component="h1" gutterBottom>
        Create New Order
      </Typography>
      
      <Paper sx={{ p: 3, mt: 2 }}>
        <Typography variant="h6" gutterBottom>
          Order Details
        </Typography>
        
        <Grid container spacing={3} sx={{ mt: 1 }}>
          <Grid size={{ xs: 12, md: 4 }}>
            <WarehouseSelect
              value={selectedWarehouse}
              onChange={handleWarehouseChange}
              required
              helperText="Select the warehouse for this order"
            />
          </Grid>
          <Grid size={{ xs: 12, md: 4 }}>
            <DistrictSelect
              warehouseId={selectedWarehouse}
              value={selectedDistrict}
              onChange={handleDistrictChange}
              required
              helperText="Select the district within the warehouse"
            />
          </Grid>
          <Grid size={{ xs: 12, md: 4 }}>
            <CustomerAutocomplete
              warehouseId={selectedWarehouse}
              districtId={selectedDistrict}
              value={selectedCustomer}
              onChange={setSelectedCustomer}
              required
              helperText="Search for a customer by name or ID"
            />
          </Grid>
        </Grid>

        {(selectedWarehouse || selectedDistrict || selectedCustomer) && (
          <Box sx={{ mt: 3, p: 2, bgcolor: 'primary.50', borderRadius: 1 }}>
            <Typography variant="body2" color="primary.main">
              {selectedWarehouse && `Selected Warehouse: ${selectedWarehouse}`}
              {selectedWarehouse && selectedDistrict && ' | '}
              {selectedDistrict && `District: ${selectedDistrict}`}
              {selectedDistrict && selectedCustomer && ' | '}
              {selectedCustomer && `Customer: ${selectedCustomer.c_first} ${selectedCustomer.c_last} (ID: ${selectedCustomer.c_id})`}
            </Typography>
          </Box>
        )}

        {/* Order Lines Section */}
        {selectedWarehouse && selectedDistrict && selectedCustomer && (
          <Box sx={{ mt: 4 }}>
            <OrderLines
              warehouseId={selectedWarehouse}
              lines={orderLines}
              onChange={setOrderLines}
            />
          </Box>
        )}
        
        {/* Order Submission Section */}
        {selectedWarehouse && selectedDistrict && selectedCustomer && (
          <Box sx={{ mt: 4 }}>
            <Divider sx={{ mb: 3 }} />
            
            {/* Validation Errors */}
            {validationErrors.length > 0 && (
              <Alert severity="error" sx={{ mb: 3 }}>
                <Typography variant="subtitle2" gutterBottom>
                  Please fix the following errors:
                </Typography>
                <ul style={{ margin: 0, paddingLeft: '20px' }}>
                  {validationErrors.map((error, index) => (
                    <li key={index}>{error}</li>
                  ))}
                </ul>
              </Alert>
            )}

            {/* API Error */}
            {orderMutation.isError && (
              <Alert severity="error" sx={{ mb: 3 }}>
                <Typography variant="subtitle2">
                  Order Submission Failed
                </Typography>
                {orderMutation.error?.message}
              </Alert>
            )}

            {/* Success Message */}
            {orderMutation.isSuccess && (
              <Alert severity="success" sx={{ mb: 3 }}>
                <Typography variant="subtitle2">
                  Order Submitted Successfully!
                </Typography>
                Order ID: {orderMutation.data?.order_id} | Total: ${parseFloat(orderMutation.data?.total_amount || '0').toFixed(2)}
              </Alert>
            )}

            {/* Order Summary & Submit */}
            <Paper sx={{ p: 3, bgcolor: 'grey.50' }}>
              <Typography variant="h6" gutterBottom>
                Order Summary
              </Typography>
              
              <Grid container spacing={2} sx={{ mb: 2 }}>
                <Grid size={{ xs: 12, sm: 6 }}>
                  <Typography variant="body2">
                    <strong>Customer:</strong> {selectedCustomer.c_first} {selectedCustomer.c_last} (ID: {selectedCustomer.c_id})
                  </Typography>
                  <Typography variant="body2">
                    <strong>Location:</strong> Warehouse {selectedWarehouse}, District {selectedDistrict}
                  </Typography>
                </Grid>
                <Grid size={{ xs: 12, sm: 6 }}>
                  <Typography variant="body2">
                    <strong>Valid Items:</strong> {validLines.length} of {orderLines.length} lines
                  </Typography>
                  <Typography variant="body1" fontWeight="medium">
                    <strong>Estimated Total:</strong> ${totalOrderValue.toFixed(2)}
                  </Typography>
                </Grid>
              </Grid>

              <Box sx={{ display: 'flex', justifyContent: 'flex-end', gap: 2 }}>
                <Button
                  variant="contained"
                  color="primary"
                  size="large"
                  onClick={handleSubmitOrder}
                  disabled={!canSubmitOrder}
                  startIcon={orderMutation.isPending ? <CircularProgress size={20} /> : <SubmitIcon />}
                >
                  {orderMutation.isPending ? 'Submitting...' : 'Submit Order'}
                </Button>
              </Box>
            </Paper>
          </Box>
        )}
      </Paper>
    </Box>
  );
}