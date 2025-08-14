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
import { useDistricts } from '../hooks/useDistricts';

interface DistrictSelectProps {
  warehouseId?: number;
  value?: number;
  onChange: (districtId: number) => void;
  disabled?: boolean;
  required?: boolean;
  error?: boolean;
  helperText?: string;
}

export function DistrictSelect({
  warehouseId,
  value,
  onChange,
  disabled = false,
  required = false,
  error = false,
  helperText,
}: DistrictSelectProps) {
  const { data: districts, isLoading, isError, error: queryError } = useDistricts(warehouseId);

  const handleChange = (event: SelectChangeEvent<number>) => {
    const districtId = event.target.value as number;
    onChange(districtId);
  };

  if (!warehouseId) {
    return (
      <FormControl fullWidth disabled={true}>
        <InputLabel>District</InputLabel>
        <Select value="" label="District">
          <MenuItem value="">
            <em>Select a warehouse first</em>
          </MenuItem>
        </Select>
        <FormHelperText>Please select a warehouse to see available districts</FormHelperText>
      </FormControl>
    );
  }

  if (isError) {
    return (
      <Alert severity="error" sx={{ mt: 1 }}>
        Failed to load districts: {queryError?.message}
      </Alert>
    );
  }

  return (
    <FormControl fullWidth disabled={disabled || isLoading} error={error} required={required}>
      <InputLabel id="district-select-label">
        District
      </InputLabel>
      <Select
        labelId="district-select-label"
        id="district-select"
        value={value || ''}
        label="District"
        onChange={handleChange}
        endAdornment={isLoading ? <CircularProgress size={20} sx={{ mr: 2 }} /> : null}
      >
        {districts?.map((district) => (
          <MenuItem key={district.d_id} value={district.d_id}>
            {district.d_id} - {district.d_name}
            {district.d_city && ` (${district.d_city})`}
          </MenuItem>
        ))}
      </Select>
      {helperText && <FormHelperText>{helperText}</FormHelperText>}
    </FormControl>
  );
}