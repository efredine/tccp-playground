import { createRootRoute, Link, Outlet } from '@tanstack/react-router';
import { TanStackRouterDevtools } from '@tanstack/react-router-devtools';
import { 
  AppBar, 
  Toolbar, 
  Typography, 
  Container, 
  Button,
  Box 
} from '@mui/material';

export const Route = createRootRoute({
  component: () => (
    <>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            TPC-C Order Management
          </Typography>
          <Box sx={{ display: 'flex', gap: 2 }}>
            <Button color="inherit" component={Link} to="/new-order">
              New Order
            </Button>
            <Button color="inherit" component={Link} to="/orders">
              Orders
            </Button>
          </Box>
        </Toolbar>
      </AppBar>
      
      <Container maxWidth="xl" sx={{ mt: 4, mb: 4 }}>
        <Outlet />
      </Container>
      
      <TanStackRouterDevtools />
    </>
  ),
});