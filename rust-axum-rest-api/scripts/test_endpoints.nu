#!/usr/bin/env nu

# Test script for TPC-C REST API endpoints
# Make sure the server is running on localhost:8080 before running this script

let base_url = "http://localhost:8080"

print "Testing TPC-C REST API endpoints..."
print "=================================="

# Test 1: Root endpoint
print "\n1. Testing root endpoint:"
try {
    let response = http get $base_url
    print $response
} catch {
    print "Failed to connect to server"
}

# Test 2: Stock level endpoint with valid parameters (this method doesn't work well with query params)
print "\n\n2. Testing stock-level endpoint with headers (likely to fail):"
try {
    let response = http get $"($base_url)/stock-level" --headers [warehouse_id 1 district_id 1 threshold 10]
    print ($response | to json --indent 2)
} catch {
    print "Request failed - headers don't work for query parameters"
}

# Alternative way with query parameters in URL
print "\n2b. Testing stock-level endpoint with URL parameters:"
try {
    let response = http get $"($base_url)/stock-level?warehouse_id=1&district_id=1&threshold=10"
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 3: Stock level endpoint without parameters (should return 400)
print "\n\n3. Testing stock-level endpoint without parameters (expect 400):"
try {
    let response = http get $"($base_url)/stock-level"
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - missing required query parameters"
}

# Test 4: Warehouses endpoint (existing)
print "\n\n4. Testing warehouses endpoint:"
try {
    let response = http get $"($base_url)/warehouses"
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

print "\n\nTesting complete!"