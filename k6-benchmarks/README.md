# TPC-C K6 Load Testing Suite

This directory contains K6 load testing scripts for the TPC-C REST API implementation. The tests validate performance and compliance with the TPC-C benchmark specification.

## 📋 Available Tests

### 1. **Full TPC-C Benchmark** (`full_tpcc_test.js`)
Complete benchmark suite with proper TPC-C transaction distribution:
- **45% New Order** - Complex multi-item order processing
- **43% Payment** - Customer payment transactions  
- **4% Order Status** - Order lookup and status reporting
- **4% Delivery** - Batch order delivery processing
- **4% Stock Level** - Inventory level analysis

### 2. **Individual Transaction Tests**
Focused tests for specific transaction types:
- `payment_test.js` - Payment transaction validation
- `delivery_test.js` - Delivery transaction testing

## 🚀 Quick Start

### Prerequisites
- [K6 installed](https://k6.io/docs/getting-started/installation/)
- TPC-C REST API server running (default: `http://localhost:8080`)
- Database populated with TPC-C data (warehouses, customers, items, etc.)

### Running Tests

#### Full TPC-C Benchmark (Recommended)
```bash
# Basic 15-second test with 10 users
k6 run full_tpcc_test.js

# Extended load test (60 seconds, 20 users)
k6 run --duration 60s --vus 20 full_tpcc_test.js

# Stress test with custom API endpoint
k6 run --duration 30s --vus 50 -e API_BASE=http://your-server:8080 full_tpcc_test.js
```

#### Individual Transaction Tests
```bash
# Test payment transactions only
k6 run payment_test.js

# Test delivery transactions with custom settings
k6 run --duration 30s --vus 5 delivery_test.js
```

## ⚙️ Configuration Options

### Environment Variables
- `API_BASE` - API server URL (default: `http://localhost:8080`)
- `WAREHOUSES` - Number of warehouses in dataset (default: `10`)

### Load Test Parameters
- `--duration` - Test duration (e.g., `30s`, `5m`, `1h`)
- `--vus` - Number of virtual users/concurrent connections
- `--iterations` - Fixed number of iterations instead of duration

### Examples
```bash
# Sysbench TPC-C equivalent (recommended baseline)
k6 run --duration 5m --vus 8 -e WAREHOUSES=10 full_tpcc_test.js

# High-load stress test
k6 run --duration 5m --vus 100 full_tpcc_test.js

# Fixed iteration count
k6 run --iterations 1000 --vus 10 full_tpcc_test.js

# Custom warehouse count and server
k6 run -e API_BASE=http://production-server:8080 -e WAREHOUSES=50 full_tpcc_test.js
```

## 📊 Understanding Results

### Key Metrics
- **Success Rate** - Should be >99% for healthy system
- **Response Time** - Average and percentiles (p90, p95)
- **Throughput** - Requests/transactions per second
- **Transaction Mix** - Distribution matches TPC-C specification

### Sample Output
```
checks_succeeded...: 99.33% 149 out of 150
http_req_duration..: avg=36ms p(90)=57ms p(95)=104ms  
http_reqs..........: 150 requests (9.6/s)
```

### What Good Results Look Like
- ✅ **>99% Success Rate** - System handles load reliably
- ✅ **<50ms Average Response Time** - Good user experience  
- ✅ **All Transaction Types Pass** - Complete TPC-C compliance
- ✅ **Consistent Performance** - No significant degradation over time

## 🔧 Troubleshooting

### Common Issues

#### Connection Errors (0% Success Rate)
```bash
# Check if server is running
curl http://localhost:8080

# Verify API endpoints
curl http://localhost:8080/warehouses
```

#### High Error Rates (>5% Failures)
- **Database Connection Issues** - Check PostgreSQL connectivity
- **Resource Exhaustion** - Reduce VUs or test duration
- **Invalid Test Data** - Ensure TPC-C data is properly loaded

#### Poor Performance (>100ms Average)
- **Database Not Indexed** - Verify TPC-C indexes are created
- **Insufficient Resources** - Check CPU/Memory usage
- **Connection Pool Limits** - Review database connection settings

### Debug Mode
```bash
# Enable HTTP debug logging
k6 run --http-debug full_tpcc_test.js

# Verbose output with detailed metrics
k6 run --verbose full_tpcc_test.js
```

## 📈 Advanced Usage

### Load Testing Profiles

#### Development Testing
```bash
# Quick validation (30 seconds, 5 users)
k6 run --duration 30s --vus 5 full_tpcc_test.js
```

#### Performance Baseline
```bash
# Standard performance test (5 minutes, 20 users)
k6 run --duration 5m --vus 20 full_tpcc_test.js
```

#### Stress Testing
```bash
# High load stress test (10 minutes, 100 users)
k6 run --duration 10m --vus 100 full_tpcc_test.js
```

#### Capacity Planning
```bash
# Gradual ramp-up test
k6 run --stage 2m:10,5m:50,2m:100,5m:100,2m:0 full_tpcc_test.js
```

### Continuous Integration
```bash
# CI/CD pipeline test with thresholds
k6 run --quiet \
  --threshold http_req_duration="avg<50,p(95)<200" \
  --threshold http_req_failed="rate<0.05" \
  full_tpcc_test.js
```

## ⚖️ Sysbench Comparison

### Equivalent Configurations

**Standard Configuration** (`sysbench_equivalent.sh`):
| Parameter | Sysbench | K6 Equivalent |
|-----------|----------|---------------|
| Warehouses | `--scale=10` | `-e WAREHOUSES=10` |
| Duration | `--time=300` (5 min) | `--duration 5m` |
| Concurrency | `--threads=8` | `--vus 8` |

### Key Differences
- **Sysbench**: Direct database access, Lua-based
- **K6**: HTTP REST API, JavaScript-based  
- **Network Overhead**: K6 includes HTTP/JSON serialization costs
- **Transaction Logic**: Both follow TPC-C specification patterns

### Performance Expectations

**Standard Config (with think time)**:
- **TPS**: ~8-10 (much lower due to 1-second think time)
- **CPU Load**: Low - simulates realistic user behavior
- **Use Case**: User experience testing, realistic load patterns

**High-Load Config (no think time)**:  
- **TPS**: ~200-600+ (targeting sysbench's 666 TPS)
- **CPU Load**: High - matches sysbench intensity
- **Latency**: ~20-60ms (higher than sysbench's 12ms due to HTTP overhead)
- **Use Case**: Maximum throughput testing, database stress testing

**Error Patterns**: Both should show similar error rates (~1% invalid items)

## 📝 Test Development

### Adding New Tests
1. Copy an existing test file (e.g., `payment_test.js`)
2. Modify the transaction logic for your use case
3. Update check conditions for validation
4. Test with low VUs first, then scale up

### Custom Transaction Mix
Modify the `TRANSACTION_WEIGHTS` array in `full_tpcc_test.js`:
```javascript
const TRANSACTION_WEIGHTS = [
  { name: 'new_order', weight: 50 },    // 50% New Order
  { name: 'payment', weight: 40 },      // 40% Payment  
  { name: 'order_status', weight: 10 }, // 10% Order Status
  // ... adjust as needed
];
```

## 🎯 TPC-C Compliance

These tests validate compliance with the official TPC-C specification:
- **Transaction Distribution** - Matches required percentages
- **Data Access Patterns** - Random selection within TPC-C constraints
- **Error Handling** - Validates 1% invalid item requirement
- **Performance Characteristics** - Measures response times and throughput

For official TPC-C benchmarking, ensure:
- Proper database sizing (warehouses, customers per district, etc.)
- Complete TPC-C dataset population
- Production-equivalent hardware and network conditions
- Extended test durations (minimum 5 minutes for valid results)

---

**📚 Further Reading:**
- [K6 Documentation](https://k6.io/docs/)
- [TPC-C Specification](http://www.tpc.org/tpcc/)
- [API Documentation](../rust-axum-rest-api/README.md)
