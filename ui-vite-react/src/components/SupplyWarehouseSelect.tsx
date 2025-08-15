import { FormControl, InputLabel, Select, MenuItem, FormHelperText } from '@mui/material';
import { useWarehouses } from '../hooks/useWarehouses';

interface SupplyWarehouseSelectProps {
  value?: number;
  onChange: (warehouseId: number) => void;
  orderWarehouseId?: number; // To highlight the default/order warehouse
  disabled?: boolean;
  error?: boolean;
  helperText?: string;
}

export function SupplyWarehouseSelect({
  value,
  onChange,
  orderWarehouseId,
  disabled = false,
  error = false,
  helperText,
}: SupplyWarehouseSelectProps) {
  const { data: warehouses, isLoading, isError } = useWarehouses();

  if (isError) {
    return (
      <FormControl fullWidth disabled>
        <InputLabel>Supply Warehouse</InputLabel>
        <Select value="" label="Supply Warehouse">
          <MenuItem value="">Error loading warehouses</MenuItem>
        </Select>
        <FormHelperText error>Failed to load warehouses</FormHelperText>
      </FormControl>
    );
  }

  return (
    <FormControl fullWidth disabled={disabled || isLoading} error={error}>
      <InputLabel>Supply WH</InputLabel>
      <Select
        value={value || ''}
        label="Supply WH"
        onChange={(e) => onChange(Number(e.target.value))}
      >
        {warehouses?.map((warehouse) => (
          <MenuItem 
            key={warehouse.w_id} 
            value={warehouse.w_id}
            sx={{
              fontWeight: warehouse.w_id === orderWarehouseId ? 'bold' : 'normal',
              backgroundColor: warehouse.w_id === orderWarehouseId ? 'action.hover' : 'transparent',
            }}
          >
            {warehouse.w_id === orderWarehouseId && '★ '}
            WH {warehouse.w_id}
            {warehouse.w_name && ` - ${warehouse.w_name}`}
            {warehouse.w_id === orderWarehouseId && ' (Order WH)'}
          </MenuItem>
        ))}
      </Select>
      <FormHelperText>
        {helperText || (orderWarehouseId ? `Default: WH ${orderWarehouseId}` : 'Select supply warehouse')}
      </FormHelperText>
    </FormControl>
  );
}