#!/bin/bash

AI_EXECUTABLE=$1

# Test soft defense scenario
OUTPUT=$(echo "BOARD
10,10,1
11,10,1
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "9,10" ] || [ "$OUTPUT" = "12,10" ]; then
    exit 0
else
    echo "Expected move '9,10' or '12,10', got: $OUTPUT"
    exit 1
fi
