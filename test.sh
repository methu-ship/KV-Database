#!/bin/bash
echo "Testing my KV Database..."

# Clean start
rm -rf data/

# Test 1: Basic set/get
echo "Test 1: Basic operations"
cargo run --bin db -- set test1 "hello" > /dev/null
result=$(cargo run --bin db -- get test1 2>/dev/null)
if [ "$result" = "hello" ]; then
    echo "Basic set/get passed"
else
    echo "Basic set/get failed: got '$result'"
fi

# Test 2: Update
echo "Test 2: Update value"
cargo run --bin db -- set test1 "updated" > /dev/null
result=$(cargo run --bin db -- get test1 2>/dev/null)
if [ "$result" = "updated" ]; then
    echo "Update passed"
else
    echo "Update failed: got '$result'"
fi

# Test 3: Multiple keys
echo "Test 3: Multiple keys"
cargo run --bin db -- set key1 "val1" > /dev/null
cargo run --bin db -- set key2 "val2" > /dev/null
result1=$(cargo run --bin db -- get key1 2>/dev/null)
result2=$(cargo run --bin db -- get key2 2>/dev/null)
if [ "$result1" = "val1" ] && [ "$result2" = "val2" ]; then
    echo "Multiple keys passed"
else
    echo " Multiple keys failed"
fi

echo "Tests completed!"


# To run just use this command: chmod +x test.sh && ./test.sh