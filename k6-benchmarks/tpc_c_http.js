import http from 'k6/http';
import { check, sleep } from 'k6';

const API_BASE = __ENV.API_BASE || 'https://your-api.example.com';
const WAREHOUSES = parseInt(__ENV.WAREHOUSES || '10');
const DISTRICTS_PER_W = 10;
const CUSTOMERS_PER_D = 3000;
const ITEMS = 100000;

// k6 scenarios to match TPC-C mix
export let options = {
  scenarios: {
    new_order: {
      executor: 'constant-vus',
      exec: 'txn_new_order',
      vus: 20,
      duration: '1m',
      startTime: '0s',
    },
    payment: {
      executor: 'constant-vus',
      exec: 'txn_payment',
      vus: 19, // ~43% of 44 total VUs
      duration: '1m',
      startTime: '0s',
    },
    order_status: {
      executor: 'constant-vus',
      exec: 'txn_order_status',
      vus: 2, // ~4%
      duration: '1m',
      startTime: '0s',
    },
    delivery: {
      executor: 'constant-vus',
      exec: 'txn_delivery',
      vus: 2,
      duration: '1m',
      startTime: '0s',
    },
    stock_level: {
      executor: 'constant-vus',
      exec: 'txn_stock_level',
      vus: 2,
      duration: '1m',
      startTime: '0s',
    },
  },
};

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

// === Transaction Implementations ===
export function txn_new_order() {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();
  const c_id = pickCustomer();
  const ol_cnt = randInt(5, 15);

  let order_lines = [];
  for (let i = 0; i < ol_cnt; i++) {
    order_lines.push({
      item_id: randInt(1, ITEMS),
      supply_w_id: w_id,
      quantity: randInt(1, 10),
    });
  }

  const res = http.post(`${API_BASE}/new-order`, JSON.stringify({
    warehouse_id: w_id,
    district_id: d_id,
    customer_id: c_id,
    order_lines: order_lines
  }), { headers: { 'Content-Type': 'application/json' } });

  check(res, { 'new_order 200': (r) => r.status === 200 });
  sleep(2); // think time
}

export function txn_payment() {
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

  check(res, { 'payment 200': (r) => r.status === 200 });
  sleep(2);
}

export function txn_order_status() {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();
  const c_id = pickCustomer();

  const res = http.get(`${API_BASE}/order-status?warehouse_id=${w_id}&district_id=${d_id}&customer_id=${c_id}`);
  check(res, { 'order_status 200': (r) => r.status === 200 });
  sleep(2);
}

export function txn_delivery() {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();

  const res = http.post(`${API_BASE}/delivery`, JSON.stringify({
    warehouse_id: w_id,
    district_id: d_id
  }), { headers: { 'Content-Type': 'application/json' } });

  check(res, { 'delivery 200': (r) => r.status === 200 });
  sleep(2);
}

export function txn_stock_level() {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();
  const threshold = randInt(10, 20);

  const res = http.get(`${API_BASE}/stock-level?warehouse_id=${w_id}&district_id=${d_id}&threshold=${threshold}`);
  check(res, { 'stock_level 200': (r) => r.status === 200 });
  sleep(2);
}
