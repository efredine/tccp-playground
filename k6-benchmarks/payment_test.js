import http from 'k6/http';
import { check, sleep } from 'k6';

const API_BASE = __ENV.API_BASE || 'http://localhost:8080';
const WAREHOUSES = parseInt(__ENV.WAREHOUSES || '10');
const DISTRICTS_PER_W = 10;
const CUSTOMERS_PER_D = 3000;

// Utility random functions
function randInt(min, max) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}
function pickWarehouse() {
  return randInt(1, WAREHOUSES);
}
function pickDistrict() {
  return randInt(1, DISTRICTS_PER_W);
}
function pickCustomer() {
  return randInt(1, CUSTOMERS_PER_D);
}

export let options = {
  vus: 5,
  duration: '10s',
};

export default function () {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();
  const c_id = pickCustomer();
  const payment_amount = randInt(1, 5000) / 100.0; // $0.01 to $50.00

  const res = http.post(`${API_BASE}/payment`, JSON.stringify({
    warehouse_id: w_id,
    district_id: d_id,
    customer_id: c_id,
    amount: payment_amount
  }), { headers: { 'Content-Type': 'application/json' } });

  check(res, { 
    'payment 200': (r) => r.status === 200,
    'payment has payment_amount': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.payment_amount !== undefined;
      } catch (e) {
        return false;
      }
    },
    'payment has customer info': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.customer && body.customer.c_id !== undefined;
      } catch (e) {
        return false;
      }
    }
  });
  
  sleep(1);
}