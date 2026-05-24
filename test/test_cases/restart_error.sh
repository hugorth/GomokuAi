#!/bin/bash
# test/test_cases/restart_error.sh

AI_EXECUTABLE=$1

# Try RESTART without initializing board first
OUTPUT=$(echo "RESTART" | "$AI_EXECUTABLE")

if [[ "$OUTPUT" == "ERROR Cannot restart: board was never initialized" ]]; then
    echo "RESTART error test passed"
    exit 0
else
    echo "RESTART error test failed: Expected 'ERROR Cannot restart: board was never initialized', got: $OUTPUT"
    exit 1
fi