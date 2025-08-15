import { createFileRoute } from '@tanstack/react-router';
import { useState } from 'react';
import { 
  Typography, 
  Paper, 
  Box,
  Grid
} from '@mui/material';
import { WarehouseSelect } from '../components/WarehouseSelect';
import { DistrictSelect } from '../components/DistrictSelect';
import { CustomerAutocomplete } from '../components/CustomerAutocomplete';
import { OrderLines, type OrderLine } from '../components/OrderLines';
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
        
        {/* Still to implement */}
        {selectedWarehouse && selectedDistrict && selectedCustomer && (
          <Box sx={{ mt: 4 }}>
            <Typography variant="body2" color="text.secondary">
              Still to implement:
            </Typography>
            <ul>
              <li>Order validation and submission</li>
            </ul>
          </Box>
        )}
      </Paper>
    </Box>
  );
}