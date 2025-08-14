#!/bin/bash

# Sysbench TPC-C Equivalent K6 Test
# 
# Sysbench config:
# --scale=10 (10 warehouses)  
# --time=300 (5 minutes)
# --threads=8 (8 concurrent users)
#
# K6 equivalent:

echo "Running K6 test equivalent to sysbench-tpcc/benchmark.sh"
echo "Configuration: 10 warehouses, 5 minutes, 8 concurrent users"
echo ""

k6 run \
  --duration 5m \
  --vus 8 \
  -e API_BASE=http://localhost:8080 \
  -e WAREHOUSES=10 \
  full_tpcc_test.js