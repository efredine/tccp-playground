import type { Customer } from '../types/order.types';
import type { OrderLine } from '../components/OrderLines';

export interface OrderValidationError {
  field: string;
  message: string;
}

export interface OrderValidationResult {
  isValid: boolean;
  errors: OrderValidationError[];
}

export const validateOrder = (
  warehouseId?: number,
  districtId?: number,
  customer?: Customer | null,
  orderLines?: OrderLine[]
): OrderValidationResult => {
  const errors: OrderValidationError[] = [];

  // Validate basic order info
  if (!warehouseId) {
    errors.push({ field: 'warehouse', message: 'Warehouse is required' });
  }

  if (!districtId) {
    errors.push({ field: 'district', message: 'District is required' });
  }

  if (!customer) {
    errors.push({ field: 'customer', message: 'Customer is required' });
  }

  // Validate order lines
  if (!orderLines || orderLines.length === 0) {
    errors.push({ field: 'orderLines', message: 'At least one order line is required' });
  } else {
    // Validate individual order lines
    orderLines.forEach((line, index) => {
      if (!line.item) {
        errors.push({ 
          field: `orderLines[${index}].item`, 
          message: `Item is required for line ${index + 1}` 
        });
      }

      if (line.quantity <= 0) {
        errors.push({ 
          field: `orderLines[${index}].quantity`, 
          message: `Quantity must be greater than 0 for line ${index + 1}` 
        });
      }

      if (line.quantity > 99) {
        errors.push({ 
          field: `orderLines[${index}].quantity`, 
          message: `Quantity cannot exceed 99 for line ${index + 1}` 
        });
      }

      if (!line.supply_w_id) {
        errors.push({ 
          field: `orderLines[${index}].supply_w_id`, 
          message: `Supply warehouse is required for line ${index + 1}` 
        });
      }
    });

    // Check for valid items (at least one line must have an item selected)
    const validLines = orderLines.filter(line => line.item);
    if (validLines.length === 0) {
      errors.push({ field: 'orderLines', message: 'At least one order line must have an item selected' });
    }

    // TPC-C spec: maximum 15 order lines
    if (orderLines.length > 15) {
      errors.push({ field: 'orderLines', message: 'Maximum 15 order lines allowed' });
    }
  }

  return {
    isValid: errors.length === 0,
    errors,
  };
};

export const formatValidationErrors = (errors: OrderValidationError[]): string => {
  return errors.map(error => error.message).join('; ');
};