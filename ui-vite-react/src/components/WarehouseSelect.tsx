import { 
  FormControl, 
  InputLabel, 
  Select, 
  MenuItem, 
  FormHelperText,
  CircularProgress,
  Alert
} from '@mui/material';
import type { SelectChangeEvent } from '@mui/material';
import { useWarehouses } from '../hooks/useWarehouses';

interface WarehouseSelectProps {
  value?: number;
  onChange: (warehouseId: number) => void;
  disabled?: boolean;
  required?: boolean;
  error?: boolean;
  helperText?: string;
}

export function WarehouseSelect({
  value,
  onChange,
  disabled = false,
  required = false,
  error = false,
  helperText,
}: WarehouseSelectProps) {
  const { data: warehouses, isLoading, isError, error: queryError } = useWarehouses();

  const handleChange = (event: SelectChangeEvent<number>) => {
    const warehouseId = event.target.value as number;
    onChange(warehouseId);
  };

  if (isError) {
    return (
      <Alert severity="error" sx={{ mt: 1 }}>
        Failed to load warehouses: {queryError?.message}
      </Alert>
    );
  }

  return (
    <FormControl fullWidth disabled={disabled || isLoading} error={error} required={required}>
      <InputLabel id="warehouse-select-label">
        Warehouse
      </InputLabel>
      <Select
        labelId="warehouse-select-label"
        id="warehouse-select"
        value={value || ''}
        label="Warehouse"
        onChange={handleChange}
        endAdornment={isLoading ? <CircularProgress size={20} sx={{ mr: 2 }} /> : null}
      >
        {warehouses?.map((warehouse) => (
          <MenuItem key={warehouse.w_id} value={warehouse.w_id}>
            {warehouse.w_id} - {warehouse.w_name}
            {warehouse.w_city && ` (${warehouse.w_city})`}
          </MenuItem>
        ))}
      </Select>
      {helperText && <FormHelperText>{helperText}</FormHelperText>}
    </FormControl>
  );
}