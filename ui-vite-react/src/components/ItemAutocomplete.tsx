import { 
  Autocomplete, 
  TextField, 
  CircularProgress, 
  Alert,
  Box,
  Typography,
  Chip
} from '@mui/material';
import { useItems } from '../hooks/useItems';
import type { Item } from '../services/itemService';
import { useState } from 'react';

interface ItemAutocompleteProps {
  warehouseId?: number;
  value?: Item | null;
  onChange: (item: Item | null) => void;
  disabled?: boolean;
  required?: boolean;
  error?: boolean;
  helperText?: string;
}

export function ItemAutocomplete({
  warehouseId,
  value,
  onChange,
  disabled = false,
  required = false,
  error = false,
  helperText,
}: ItemAutocompleteProps) {
  const [inputValue, setInputValue] = useState('');
  const { data: items, isSearching, isError, error: queryError } = useItems(
    warehouseId,
    inputValue
  );

  const options = items || [];

  if (isError) {
    return (
      <Alert severity="error" sx={{ mt: 1 }}>
        Failed to load items: {queryError?.message}
      </Alert>
    );
  }

  const isDisabled = disabled || !warehouseId;

  const formatPrice = (price?: string | null) => {
    if (!price) return 'N/A';
    const num = parseFloat(price);
    return isNaN(num) ? 'N/A' : `$${num.toFixed(2)}`;
  };

  return (
    <Autocomplete
      value={value}
      onChange={(_, newValue: Item | null) => {
        onChange(newValue);
      }}
      inputValue={inputValue}
      onInputChange={(_, newInputValue) => {
        setInputValue(newInputValue);
      }}
      options={options}
      getOptionLabel={(option) => 
        `${option.i_name || `Item ${option.i_id}`} (ID: ${option.i_id})`
      }
      isOptionEqualToValue={(option, val) => option.i_id === val.i_id}
      loading={isSearching}
      disabled={isDisabled}
      renderInput={(params) => (
        <TextField
          {...params}
          label="Item"
          required={required}
          error={error}
          helperText={helperText || (isDisabled ? 'Select warehouse first' : 'Search by item name or ID')}
          InputProps={{
            ...params.InputProps,
            endAdornment: (
              <>
                {isSearching ? <CircularProgress color="inherit" size={20} /> : null}
                {params.InputProps.endAdornment}
              </>
            ),
          }}
        />
      )}
      renderOption={(props, option) => (
        <Box component="li" {...props} key={option.i_id}>
          <Box sx={{ width: '100%' }}>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 0.5 }}>
              <Typography variant="body2" fontWeight="medium">
                {option.i_name || `Item ${option.i_id}`}
              </Typography>
              <Chip 
                label={formatPrice(option.i_price)} 
                size="small" 
                color="primary" 
                variant="outlined"
              />
            </Box>
            <Typography variant="caption" color="text.secondary">
              ID: {option.i_id}
              {option.i_data && ` | ${option.i_data.substring(0, 50)}${option.i_data.length > 50 ? '...' : ''}`}
            </Typography>
          </Box>
        </Box>
      )}
      noOptionsText={isSearching ? "Searching..." : "No items found"}
    />
  );
}