import { createFileRoute } from '@tanstack/react-router';
import { 
  Typography, 
  Card, 
  CardContent, 
  Grid, 
  Button,
  Box 
} from '@mui/material';
import { Link } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Index,
});

function Index() {
  return (
    <Box>
      <Typography variant="h4" component="h1" gutterBottom>
        Welcome to TPC-C Order Management
      </Typography>
      
      <Typography variant="body1" paragraph>
        A high-performance order management system built on the TPC-C benchmark specification.
        This interface provides access to order creation, viewing, and management capabilities.
      </Typography>

      <Grid container spacing={3} sx={{ mt: 2 }}>
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h5" component="h2" gutterBottom>
                New Order
              </Typography>
              <Typography variant="body2" color="text.secondary" paragraph>
                Create new customer orders with multiple line items.
                Select warehouse, district, customer, and items.
              </Typography>
              <Button 
                variant="contained" 
                component={Link} 
                to="/new-order"
                fullWidth
              >
                Create New Order
              </Button>
            </CardContent>
          </Card>
        </Grid>
        
        <Grid item xs={12} md={6}>
          <Card>
            <CardContent>
              <Typography variant="h5" component="h2" gutterBottom>
                View Orders
              </Typography>
              <Typography variant="body2" color="text.secondary" paragraph>
                Browse existing orders with filtering by warehouse, 
                district, customer, and order status.
              </Typography>
              <Button 
                variant="contained" 
                component={Link} 
                to="/orders"
                fullWidth
              >
                View Orders
              </Button>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
}