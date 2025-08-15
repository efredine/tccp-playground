import { 
  Autocomplete, 
  TextField, 
  CircularProgress, 
  Alert,
  Box,
  Typography
} from '@mui/material';
import { useCustomers } from '../hooks/useCustomers';
import type { Customer } from '../types/order.types';
import { useState } from 'react';

interface CustomerAutocompleteProps {
  warehouseId?: number;
  districtId?: number;
  value?: Customer | null;
  onChange: (customer: Customer | null) => void;
  disabled?: boolean;
  required?: boolean;
  error?: boolean;
  helperText?: string;
}

export function CustomerAutocomplete({
  warehouseId,
  districtId,
  value,
  onChange,
  disabled = false,
  required = false,
  error = false,
  helperText,
}: CustomerAutocompleteProps) {
  const [inputValue, setInputValue] = useState('');
  const { data: customers, isFetching, isSearching, isError, error: queryError } = useCustomers(
    warehouseId,
    districtId,
    inputValue
  );

  // Debug logging (can be removed in production)
  // console.log({ inputValue, isSearching, customersCount: customers?.length || 0 });

  const options = customers || [];

  if (isError) {
    return (
      <Alert severity="error" sx={{ mt: 1 }}>
        Failed to load customers: {queryError?.message}
      </Alert>
    );
  }

  const isDisabled = disabled || !warehouseId || !districtId;

  return (
    <Autocomplete
      value={value}
      onChange={(_, newValue: Customer | null) => {
        onChange(newValue);
      }}
      inputValue={inputValue}
      onInputChange={(_, newInputValue) => {
        setInputValue(newInputValue);
      }}
      options={options}
      getOptionLabel={(option) => 
        `${option.c_first || ''} ${option.c_middle || ''} ${option.c_last || ''} (ID: ${option.c_id})`.trim()
      }
      isOptionEqualToValue={(option, val) => option.c_id === val.c_id}
      loading={isSearching}
      disabled={isDisabled}
      renderInput={(params) => (
        <TextField
          {...params}
          label="Customer"
          required={required}
          error={error}
          helperText={helperText || (isDisabled ? 'Select warehouse and district first' : 'Type to search for a customer')}
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
        <Box component="li" {...props} key={option.c_id}>
          <Box>
            <Typography variant="body2" fontWeight="medium">
              {option.c_first} {option.c_middle} {option.c_last}
            </Typography>
            <Typography variant="caption" color="text.secondary">
              ID: {option.c_id} | {option.c_city}, {option.c_state}
            </Typography>
          </Box>
        </Box>
      )}
      noOptionsText={isSearching ? "Searching..." : "No customers found"}
    />
  );
}
