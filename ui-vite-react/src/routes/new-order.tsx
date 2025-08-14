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

export const Route = createFileRoute('/new-order')({
  component: NewOrderPage,
});

function NewOrderPage() {
  const [selectedWarehouse, setSelectedWarehouse] = useState<number>();
  const [selectedDistrict, setSelectedDistrict] = useState<number>();

  // Reset district when warehouse changes
  const handleWarehouseChange = (warehouseId: number) => {
    setSelectedWarehouse(warehouseId);
    setSelectedDistrict(undefined); // Clear district selection
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
              onChange={setSelectedDistrict}
              required
              helperText="Select the district within the warehouse"
            />
          </Grid>
        </Grid>

        {(selectedWarehouse || selectedDistrict) && (
          <Box sx={{ mt: 3, p: 2, bgcolor: 'primary.50', borderRadius: 1 }}>
            <Typography variant="body2" color="primary.main">
              {selectedWarehouse && `Selected Warehouse: ${selectedWarehouse}`}
              {selectedWarehouse && selectedDistrict && ' | '}
              {selectedDistrict && `District: ${selectedDistrict}`}
            </Typography>
          </Box>
        )}
        
        <Typography variant="body2" color="text.secondary" sx={{ mt: 4 }}>
          Still to implement:
        </Typography>
        <ul>
          <li>District selection (depends on warehouse)</li>
          <li>Customer search/selection</li>
          <li>Dynamic order lines with item search</li>
          <li>Real-time price calculation</li>
          <li>Order validation and submission</li>
        </ul>
      </Paper>
    </Box>
  );
}