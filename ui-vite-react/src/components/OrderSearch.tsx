import { useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  TextField,
  MenuItem,
  Button,
  Grid,
  Typography,
  Collapse,
  IconButton,
  Chip,
} from '@mui/material';
import {
  Search as SearchIcon,
  FilterList as FilterIcon,
  Clear as ClearIcon,
} from '@mui/icons-material';
import type { OrdersQuery } from '../types/orders';

interface OrderSearchProps {
  onSearch: (query: OrdersQuery) => void;
  isLoading?: boolean;
}

export default function OrderSearch({ onSearch, isLoading = false }: OrderSearchProps) {
  const [query, setQuery] = useState<OrdersQuery>({
    page: 1,
    per_page: 20,
    sort_by: 'entry_date',
    sort_dir: 'desc',
  });
  
  const [showFilters, setShowFilters] = useState(false);
  const [hasActiveFilters, setHasActiveFilters] = useState(false);

  const handleInputChange = (field: keyof OrdersQuery, value: string | number | undefined) => {
    const newQuery = { ...query, [field]: value === '' ? undefined : value };
    setQuery(newQuery);
    
    // Update active filters indicator
    const activeFilters = [
      newQuery.warehouse_id,
      newQuery.district_id,
      newQuery.customer_id,
      newQuery.order_id,
      newQuery.from_date,
      newQuery.to_date,
    ].filter(Boolean);
    setHasActiveFilters(activeFilters.length > 0);
  };

  const handleSearch = () => {
    // Reset to first page when searching
    const searchQuery = { ...query, page: 1 };
    setQuery(searchQuery);
    onSearch(searchQuery);
  };

  const handleClearFilters = () => {
    const clearedQuery = {
      page: 1,
      per_page: query.per_page,
      sort_by: query.sort_by,
      sort_dir: query.sort_dir,
    };
    setQuery(clearedQuery);
    setHasActiveFilters(false);
    onSearch(clearedQuery);
  };

  const sortByOptions = [
    { value: 'entry_date', label: 'Entry Date' },
    { value: 'order_id', label: 'Order ID' },
    { value: 'customer_last', label: 'Customer Last Name' },
    { value: 'warehouse_id', label: 'Warehouse ID' },
    { value: 'district_id', label: 'District ID' },
    { value: 'carrier_id', label: 'Carrier ID' },
  ];

  const sortDirOptions = [
    { value: 'desc', label: 'Descending' },
    { value: 'asc', label: 'Ascending' },
  ];

  return (
    <Card elevation={2}>
      <CardContent>
        <Box display="flex" alignItems="center" gap={2} mb={2}>
          <SearchIcon color="primary" />
          <Typography variant="h6" component="h2" flexGrow={1}>
            Order Search
          </Typography>
          <Box display="flex" alignItems="center" gap={1}>
            {hasActiveFilters && (
              <Chip 
                label="Filters Active" 
                color="primary" 
                size="small"
                onDelete={handleClearFilters}
                deleteIcon={<ClearIcon />}
              />
            )}
            <IconButton
              onClick={() => setShowFilters(!showFilters)}
              color={showFilters ? 'primary' : 'default'}
              size="small"
            >
              <FilterIcon />
            </IconButton>
          </Box>
        </Box>

        {/* Quick Search Row */}
        <Grid container spacing={2} sx={{ alignItems: 'center', mb: 2 }}>
          <Grid size={{ xs: 12, sm: 3 }}>
            <TextField
              fullWidth
              label="Order ID"
              type="number"
              value={query.order_id || ''}
              onChange={(e) => handleInputChange('order_id', e.target.value ? parseInt(e.target.value) : undefined)}
              size="small"
            />
          </Grid>
          <Grid size={{ xs: 12, sm: 3 }}>
            <TextField
              fullWidth
              label="Customer ID"
              type="number"
              value={query.customer_id || ''}
              onChange={(e) => handleInputChange('customer_id', e.target.value ? parseInt(e.target.value) : undefined)}
              size="small"
            />
          </Grid>
          <Grid size={{ xs: 12, sm: 3 }}>
            <TextField
              select
              fullWidth
              label="Sort By"
              value={query.sort_by || 'entry_date'}
              onChange={(e) => handleInputChange('sort_by', e.target.value as OrdersQuery['sort_by'])}
              size="small"
            >
              {sortByOptions.map((option) => (
                <MenuItem key={option.value} value={option.value}>
                  {option.label}
                </MenuItem>
              ))}
            </TextField>
          </Grid>
          <Grid size={{ xs: 12, sm: 3 }}>
            <TextField
              select
              fullWidth
              label="Sort Direction"
              value={query.sort_dir || 'desc'}
              onChange={(e) => handleInputChange('sort_dir', e.target.value as OrdersQuery['sort_dir'])}
              size="small"
            >
              {sortDirOptions.map((option) => (
                <MenuItem key={option.value} value={option.value}>
                  {option.label}
                </MenuItem>
              ))}
            </TextField>
          </Grid>
        </Grid>

        {/* Advanced Filters */}
        <Collapse in={showFilters}>
          <Box mt={2} p={2} border={1} borderColor="divider" borderRadius={1}>
            <Typography variant="subtitle2" color="text.secondary" gutterBottom>
              Advanced Filters
            </Typography>
            
            <Grid container spacing={2}>
              <Grid size={{ xs: 12, sm: 6, md: 3 }}>
                <TextField
                  fullWidth
                  label="Warehouse ID"
                  type="number"
                  value={query.warehouse_id || ''}
                  onChange={(e) => handleInputChange('warehouse_id', e.target.value ? parseInt(e.target.value) : undefined)}
                  size="small"
                />
              </Grid>
              <Grid size={{ xs: 12, sm: 6, md: 3 }}>
                <TextField
                  fullWidth
                  label="District ID"
                  type="number"
                  value={query.district_id || ''}
                  onChange={(e) => handleInputChange('district_id', e.target.value ? parseInt(e.target.value) : undefined)}
                  size="small"
                />
              </Grid>
              <Grid size={{ xs: 12, sm: 6, md: 3 }}>
                <TextField
                  fullWidth
                  label="From Date"
                  type="date"
                  value={query.from_date || ''}
                  onChange={(e) => handleInputChange('from_date', e.target.value || undefined)}
                  InputLabelProps={{ shrink: true }}
                  size="small"
                />
              </Grid>
              <Grid size={{ xs: 12, sm: 6, md: 3 }}>
                <TextField
                  fullWidth
                  label="To Date"
                  type="date"
                  value={query.to_date || ''}
                  onChange={(e) => handleInputChange('to_date', e.target.value || undefined)}
                  InputLabelProps={{ shrink: true }}
                  size="small"
                />
              </Grid>
            </Grid>
          </Box>
        </Collapse>

        {/* Action Buttons */}
        <Box display="flex" gap={2} mt={2}>
          <Button
            variant="contained"
            startIcon={<SearchIcon />}
            onClick={handleSearch}
            disabled={isLoading}
          >
            Search Orders
          </Button>
          
          {hasActiveFilters && (
            <Button
              variant="outlined"
              startIcon={<ClearIcon />}
              onClick={handleClearFilters}
              disabled={isLoading}
            >
              Clear Filters
            </Button>
          )}
        </Box>
      </CardContent>
    </Card>
  );
}
