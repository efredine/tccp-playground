import {
  Box,
  Typography,
  Button,
  Paper,
  Grid,
  TextField,
  IconButton,
  Divider,
  Alert,
} from '@mui/material';
import { Add as AddIcon, Delete as DeleteIcon } from '@mui/icons-material';
import { useCallback } from 'react';
import { ItemAutocomplete } from './ItemAutocomplete';
import { SupplyWarehouseSelect } from './SupplyWarehouseSelect';
import { useStockInfo } from '../hooks/useItems';
import type { Item } from '../services/itemService';

export interface OrderLine {
  id: string; // Temporary ID for React keys
  item?: Item | null;
  quantity: number;
  supply_w_id?: number; // Supply warehouse (defaults to order warehouse)
}

interface OrderLinesProps {
  warehouseId?: number;
  lines: OrderLine[];
  onChange: (lines: OrderLine[]) => void;
}

function OrderLineRow({ 
  line, 
  warehouseId, 
  onUpdate, 
  onDelete, 
  canDelete 
}: {
  line: OrderLine;
  warehouseId?: number;
  onUpdate: (updates: Partial<OrderLine>) => void;
  onDelete: () => void;
  canDelete: boolean;
}) {
  const { data: stockInfo, isLoading: stockLoading } = useStockInfo(
    line.supply_w_id || warehouseId,
    line.item?.i_id
  );

  const totalPrice = line.item?.i_price && line.quantity > 0
    ? parseFloat(line.item.i_price) * line.quantity
    : 0;

  const isOutOfStock = stockInfo && stockInfo.s_quantity < line.quantity;

  return (
    <Paper sx={{ p: 2, mb: 2 }}>
      <Grid container spacing={2} alignItems="flex-start">
        <Grid size={{ xs: 12, md: 4.5 }}>
          <ItemAutocomplete
            warehouseId={warehouseId}
            value={line.item}
            onChange={(item) => onUpdate({ item })}
            helperText="Search for an item"
          />
        </Grid>
        
        <Grid size={{ xs: 4, md: 1.5 }}>
          <TextField
            label="Quantity"
            type="number"
            value={line.quantity}
            onChange={(e) => onUpdate({ quantity: Math.max(1, parseInt(e.target.value) || 1) })}
            inputProps={{ min: 1, max: 99 }}
            fullWidth
            disabled={!line.item}
          />
        </Grid>

        <Grid size={{ xs: 4, md: 2.5 }}>
          <SupplyWarehouseSelect
            value={line.supply_w_id || warehouseId}
            onChange={(supplyWarehouseId) => onUpdate({ supply_w_id: supplyWarehouseId })}
            orderWarehouseId={warehouseId}
            disabled={!line.item}
            helperText="Optional override"
          />
        </Grid>

        <Grid size={{ xs: 4, md: 2.5 }}>
          {line.item ? (
            <Box sx={{ pt: 1 }}>
              <Typography variant="body2" color="text.secondary">
                Unit: ${parseFloat(line.item.i_price || '0').toFixed(2)}
              </Typography>
              <Typography variant="body1" fontWeight="medium" color="primary">
                Total: ${totalPrice.toFixed(2)}
              </Typography>
              {stockLoading && (
                <Typography variant="caption" color="text.secondary">
                  Checking stock...
                </Typography>
              )}
              {stockInfo && (
                <Typography 
                  variant="caption" 
                  color={isOutOfStock ? 'error' : 'success.main'}
                  display="block"
                >
                  Stock: {stockInfo.s_quantity} available
                </Typography>
              )}
            </Box>
          ) : (
            <Box sx={{ pt: 1 }}>
              <Typography variant="body2" color="text.disabled">
                Select an item to see pricing
              </Typography>
            </Box>
          )}
        </Grid>

        <Grid size={{ xs: 12, md: 1 }} sx={{ display: 'flex', justifyContent: 'center', alignItems: 'flex-start', pt: 1 }}>
          <IconButton 
            onClick={onDelete} 
            disabled={!canDelete}
            color="error"
            size="small"
          >
            <DeleteIcon />
          </IconButton>
        </Grid>
      </Grid>

      {isOutOfStock && (
        <Alert severity="warning" sx={{ mt: 1 }}>
          Only {stockInfo?.s_quantity} units available, but {line.quantity} requested
        </Alert>
      )}
    </Paper>
  );
}

export function OrderLines({ warehouseId, lines, onChange }: OrderLinesProps) {
  const addLine = useCallback(() => {
    const newLine: OrderLine = {
      id: `line-${Date.now()}-${Math.random()}`,
      item: null,
      quantity: 1,
      supply_w_id: warehouseId,
    };
    onChange([...lines, newLine]);
  }, [lines, onChange, warehouseId]);

  const updateLine = useCallback((index: number, updates: Partial<OrderLine>) => {
    const newLines = [...lines];
    newLines[index] = { ...newLines[index], ...updates };
    onChange(newLines);
  }, [lines, onChange]);

  const deleteLine = useCallback((index: number) => {
    if (lines.length > 1) {
      const newLines = lines.filter((_, i) => i !== index);
      onChange(newLines);
    }
  }, [lines, onChange]);

  const totalOrderValue = lines.reduce((sum, line) => {
    if (line.item?.i_price && line.quantity > 0) {
      return sum + (parseFloat(line.item.i_price) * line.quantity);
    }
    return sum;
  }, 0);

  const validLinesCount = lines.filter(line => line.item).length;

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
        <Box>
          <Typography variant="h6">
            Order Items ({lines.length} line{lines.length !== 1 ? 's' : ''})
          </Typography>
          <Typography variant="caption" color="text.secondary">
            Supply warehouse defaults to order warehouse (WH {warehouseId}). Override per item if needed.
          </Typography>
        </Box>
        <Button
          startIcon={<AddIcon />}
          onClick={addLine}
          variant="outlined"
          disabled={!warehouseId}
        >
          Add Item
        </Button>
      </Box>

      {lines.map((line, index) => (
        <OrderLineRow
          key={line.id}
          line={line}
          warehouseId={warehouseId}
          onUpdate={(updates) => updateLine(index, updates)}
          onDelete={() => deleteLine(index)}
          canDelete={lines.length > 1}
        />
      ))}

      <Divider sx={{ my: 2 }} />
      
      <Box sx={{ p: 2, bgcolor: 'grey.50', borderRadius: 1 }}>
        <Grid container spacing={2}>
          <Grid size={{ xs: 12, sm: 6 }}>
            <Typography variant="body2">
              Valid Items: {validLinesCount} of {lines.length}
            </Typography>
          </Grid>
          <Grid size={{ xs: 12, sm: 6 }}>
            <Typography variant="body1" fontWeight="medium" textAlign="right">
              Total Order Value: ${totalOrderValue.toFixed(2)}
            </Typography>
          </Grid>
        </Grid>
      </Box>
    </Box>
  );
}
