import { useMutation } from '@tanstack/react-query';
import { submitNewOrder, type NewOrderRequest, type NewOrderResponse } from '../services/orderService';

export const useOrderSubmission = () => {
  return useMutation<NewOrderResponse, Error, NewOrderRequest>({
    mutationFn: submitNewOrder,
    onSuccess: (data) => {
      console.log('Order submitted successfully:', data);
    },
    onError: (error) => {
      console.error('Order submission failed:', error);
    },
  });
};