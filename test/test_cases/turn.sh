#!/bin/bash

# test/test_cases/turn.sh

AI_EXECUTABLE=$1

# Test a simple TURN command
OUTPUT=$(echo "TURN 10,10" | "$AI_EXECUTABLE")

# Check if output is in valid format (two numbers separated by comma)
if [[ "$OUTPUT" =~ ^[0-9]+,[0-9]+$ ]]; then
    echo "Valid move: $OUTPUT"
    exit 0
else
    echo "Invalid move format: $OUTPUT"
    echo "Expected format: x,y (e.g., 11,10)"
    exit 1
fi
