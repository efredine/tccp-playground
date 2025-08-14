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

# Test 5: Order status endpoint with valid parameters
print "\n\n5. Testing order-status endpoint with valid parameters:"
try {
    let response = http get $"($base_url)/order-status?warehouse_id=1&district_id=1&customer_id=1"
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 6: Order status endpoint with different customer (to test variation)
print "\n\n6. Testing order-status endpoint with customer_id=100:"
try {
    let response = http get $"($base_url)/order-status?warehouse_id=1&district_id=1&customer_id=100"
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 7: Order status endpoint without parameters (should return 400)
print "\n\n7. Testing order-status endpoint without parameters (expect 400):"
try {
    let response = http get $"($base_url)/order-status"
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - missing required query parameters"
}

# Test 8: Order status endpoint with non-existent customer (should return 404)
print "\n\n8. Testing order-status endpoint with non-existent customer (expect 404):"
try {
    let response = http get $"($base_url)/order-status?warehouse_id=999&district_id=999&customer_id=999999"
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - customer not found"
}

# Test 9: New Order endpoint - valid request with single item
print "\n\n9. Testing new-order endpoint with valid single item request:"
try {
    let new_order_payload = {
        "warehouse_id": 1,
        "district_id": 1,
        "customer_id": 1,
        "order_lines": [
            {
                "item_id": 1,
                "supply_warehouse_id": 1,
                "quantity": 5
            }
        ]
    }
    let response = http post $"($base_url)/new-order" --content-type "application/json" ($new_order_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 10: New Order endpoint - valid request with multiple items
print "\n\n10. Testing new-order endpoint with valid multi-item request:"
try {
    let new_order_payload = {
        "warehouse_id": 1,
        "district_id": 1,
        "customer_id": 2,
        "order_lines": [
            {
                "item_id": 1,
                "supply_warehouse_id": 1,
                "quantity": 3
            },
            {
                "item_id": 2,
                "supply_warehouse_id": 1,
                "quantity": 2
            },
            {
                "item_id": 5,
                "supply_warehouse_id": 2,
                "quantity": 1
            }
        ]
    }
    let response = http post $"($base_url)/new-order" --content-type "application/json" ($new_order_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 11: New Order endpoint - invalid request with no order lines (should return 400)
print "\n\n11. Testing new-order endpoint with empty order lines (expect 400):"
try {
    let new_order_payload = {
        "warehouse_id": 1,
        "district_id": 1,
        "customer_id": 1,
        "order_lines": []
    }
    let response = http post $"($base_url)/new-order" --content-type "application/json" ($new_order_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - empty order lines"
}

# Test 12: New Order endpoint - invalid request with non-existent item (should return 404)
print "\n\n12. Testing new-order endpoint with non-existent item (expect 404):"
try {
    let new_order_payload = {
        "warehouse_id": 1,
        "district_id": 1,
        "customer_id": 1,
        "order_lines": [
            {
                "item_id": 999999,
                "supply_warehouse_id": 1,
                "quantity": 1
            }
        ]
    }
    let response = http post $"($base_url)/new-order" --content-type "application/json" ($new_order_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - item not found"
}

# Test 13: New Order endpoint - invalid request with non-existent customer (should return 404)
print "\n\n13. Testing new-order endpoint with non-existent customer (expect 404):"
try {
    let new_order_payload = {
        "warehouse_id": 1,
        "district_id": 1,
        "customer_id": 999999,
        "order_lines": [
            {
                "item_id": 1,
                "supply_warehouse_id": 1,
                "quantity": 1
            }
        ]
    }
    let response = http post $"($base_url)/new-order" --content-type "application/json" ($new_order_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - customer not found"
}

# Test 14: Payment endpoint - valid payment request
print "\n\n14. Testing payment endpoint with valid payment request:"
try {
    let payment_payload = {
        "warehouse_id": 1,
        "district_id": 1,
        "customer_id": 1,
        "amount": 25.50
    }
    let response = http post $"($base_url)/payment" --content-type "application/json" ($payment_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 15: Payment endpoint - different customer payment
print "\n\n15. Testing payment endpoint with different customer:"
try {
    let payment_payload = {
        "warehouse_id": 1,
        "district_id": 2,
        "customer_id": 100,
        "amount": 15.25
    }
    let response = http post $"($base_url)/payment" --content-type "application/json" ($payment_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 16: Payment endpoint - invalid amount (negative) (should return 400)
print "\n\n16. Testing payment endpoint with negative amount (expect 400):"
try {
    let payment_payload = {
        "warehouse_id": 1,
        "district_id": 1,
        "customer_id": 1,
        "amount": -10.00
    }
    let response = http post $"($base_url)/payment" --content-type "application/json" ($payment_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - negative payment amount"
}

# Test 17: Payment endpoint - zero amount (should return 400)
print "\n\n17. Testing payment endpoint with zero amount (expect 400):"
try {
    let payment_payload = {
        "warehouse_id": 1,
        "district_id": 1,
        "customer_id": 1,
        "amount": 0.00
    }
    let response = http post $"($base_url)/payment" --content-type "application/json" ($payment_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - zero payment amount"
}

# Test 18: Payment endpoint - non-existent customer (should return 404)
print "\n\n18. Testing payment endpoint with non-existent customer (expect 404):"
try {
    let payment_payload = {
        "warehouse_id": 1,
        "district_id": 1,
        "customer_id": 999999,
        "amount": 10.00
    }
    let response = http post $"($base_url)/payment" --content-type "application/json" ($payment_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - customer not found"
}

# Test 19: Payment endpoint - non-existent warehouse (should return 404)
print "\n\n19. Testing payment endpoint with non-existent warehouse (expect 404):"
try {
    let payment_payload = {
        "warehouse_id": 999,
        "district_id": 1,
        "customer_id": 1,
        "amount": 10.00
    }
    let response = http post $"($base_url)/payment" --content-type "application/json" ($payment_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "HTTP Error (expected) - warehouse not found"
}

# Test 20: Delivery endpoint - valid delivery request
print "\n\n20. Testing delivery endpoint with valid delivery request:"
try {
    let delivery_payload = {
        "warehouse_id": 1,
        "district_id": 1
    }
    let response = http post $"($base_url)/delivery" --content-type "application/json" ($delivery_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 21: Delivery endpoint - different district
print "\n\n21. Testing delivery endpoint with different district:"
try {
    let delivery_payload = {
        "warehouse_id": 1,
        "district_id": 2
    }
    let response = http post $"($base_url)/delivery" --content-type "application/json" ($delivery_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 22: Delivery endpoint - different warehouse
print "\n\n22. Testing delivery endpoint with different warehouse:"
try {
    let delivery_payload = {
        "warehouse_id": 2,
        "district_id": 1
    }
    let response = http post $"($base_url)/delivery" --content-type "application/json" ($delivery_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

# Test 23: Delivery endpoint - non-existent warehouse (should still return 200 with 0 deliveries)
print "\n\n23. Testing delivery endpoint with non-existent warehouse:"
try {
    let delivery_payload = {
        "warehouse_id": 999,
        "district_id": 1
    }
    let response = http post $"($base_url)/delivery" --content-type "application/json" ($delivery_payload | to json)
    print ($response | to json --indent 2)
} catch {
    print "Request failed"
}

print "\n\nTesting complete!"