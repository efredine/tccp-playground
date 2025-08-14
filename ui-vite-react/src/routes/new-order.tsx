import { createFileRoute } from '@tanstack/react-router';
import { 
  Typography, 
  Paper, 
  Box 
} from '@mui/material';

export const Route = createFileRoute('/new-order')({
  component: NewOrderPage,
});

function NewOrderPage() {
  return (
    <Box>
      <Typography variant="h4" component="h1" gutterBottom>
        Create New Order
      </Typography>
      
      <Paper sx={{ p: 3, mt: 2 }}>
        <Typography variant="body1">
          New Order form will be implemented here.
        </Typography>
        
        {/* TODO: Implement NewOrderForm component */}
        <Typography variant="body2" color="text.secondary" sx={{ mt: 2 }}>
          This will include:
        </Typography>
        <ul>
          <li>Warehouse selection</li>
          <li>District selection</li>
          <li>Customer search/selection</li>
          <li>Dynamic order lines with item search</li>
          <li>Real-time price calculation</li>
          <li>Order validation and submission</li>
        </ul>
      </Paper>
    </Box>
  );
}