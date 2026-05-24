#!/bin/bash

AI_EXECUTABLE=$1

# Test edge defense scenario
OUTPUT=$(echo "BOARD
0,0,1
0,1,1
0,2,1
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "0,3" ]; then
    exit 0
else
    echo "Expected move '0,3', got: $OUTPUT"
    exit 1
fi
