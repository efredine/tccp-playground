import http from 'k6/http';
import { check, sleep } from 'k6';

const API_BASE = __ENV.API_BASE || 'http://localhost:8080';
const WAREHOUSES = parseInt(__ENV.WAREHOUSES || '10');
const DISTRICTS_PER_W = 10;
const CUSTOMERS_PER_D = 3000;
const ITEMS = 100000;

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
  vus: 10,
  duration: '15s',
};

// Transaction distribution based on TPC-C specification
// 45% New Order, 43% Payment, 4% Order Status, 4% Delivery, 4% Stock Level
const TRANSACTION_WEIGHTS = [
  { name: 'new_order', weight: 45 },
  { name: 'payment', weight: 43 },
  { name: 'order_status', weight: 4 },
  { name: 'delivery', weight: 4 },
  { name: 'stock_level', weight: 4 },
];

function selectTransaction() {
  const rand = randInt(1, 100);
  let cumulative = 0;
  
  for (const txn of TRANSACTION_WEIGHTS) {
    cumulative += txn.weight;
    if (rand <= cumulative) {
      return txn.name;
    }
  }
  return 'new_order'; // fallback
}

export default function () {
  const txnType = selectTransaction();
  
  switch (txnType) {
    case 'new_order':
      runNewOrder();
      break;
    case 'payment':
      runPayment();
      break;
    case 'order_status':
      runOrderStatus();
      break;
    case 'delivery':
      runDelivery();
      break;
    case 'stock_level':
      runStockLevel();
      break;
  }
  
  // No sleep - run transactions as fast as possible (sysbench equivalent)
}

function runNewOrder() {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();
  const c_id = pickCustomer();
  const ol_cnt = randInt(5, 15);

  let order_lines = [];
  for (let i = 0; i < ol_cnt; i++) {
    order_lines.push({
      item_id: randInt(1, ITEMS),
      supply_warehouse_id: w_id,
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
}

function runPayment() {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();
  const c_id = pickCustomer();
  const payment_amount = randInt(1, 5000) / 100.0;

  const res = http.post(`${API_BASE}/payment`, JSON.stringify({
    warehouse_id: w_id,
    district_id: d_id,
    customer_id: c_id,
    amount: payment_amount
  }), { headers: { 'Content-Type': 'application/json' } });

  check(res, { 'payment 200': (r) => r.status === 200 });
}

function runOrderStatus() {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();
  const c_id = pickCustomer();

  const res = http.get(`${API_BASE}/order-status?warehouse_id=${w_id}&district_id=${d_id}&customer_id=${c_id}`);
  check(res, { 'order_status 200': (r) => r.status === 200 });
}

function runDelivery() {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();

  const res = http.post(`${API_BASE}/delivery`, JSON.stringify({
    warehouse_id: w_id,
    district_id: d_id
  }), { headers: { 'Content-Type': 'application/json' } });

  check(res, { 'delivery 200': (r) => r.status === 200 });
}

function runStockLevel() {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();
  const threshold = randInt(10, 20);

  const res = http.get(`${API_BASE}/stock-level?warehouse_id=${w_id}&district_id=${d_id}&threshold=${threshold}`);
  check(res, { 'stock_level 200': (r) => r.status === 200 });
}