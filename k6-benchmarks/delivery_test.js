import http from 'k6/http';
import { check, sleep } from 'k6';

const API_BASE = __ENV.API_BASE || 'http://localhost:8080';
const WAREHOUSES = parseInt(__ENV.WAREHOUSES || '10');
const DISTRICTS_PER_W = 10;

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

export let options = {
  vus: 3,
  duration: '10s',
};

export default function () {
  const w_id = pickWarehouse();
  const d_id = pickDistrict();

  const res = http.post(`${API_BASE}/delivery`, JSON.stringify({
    warehouse_id: w_id,
    district_id: d_id
  }), { headers: { 'Content-Type': 'application/json' } });

  check(res, { 
    'delivery 200': (r) => r.status === 200,
    'delivery has delivery_date': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.delivery_date !== undefined;
      } catch (e) {
        return false;
      }
    },
    'delivery has total_orders_delivered': (r) => {
      try {
        const body = JSON.parse(r.body);
        return body.total_orders_delivered !== undefined;
      } catch (e) {
        return false;
      }
    }
  });
  
  sleep(1);
}