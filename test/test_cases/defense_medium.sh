#!/bin/bash

AI_EXECUTABLE=$1

# Test medium threat defense
OUTPUT=$(echo "BOARD
5,5,1
6,5,1
8,5,1
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "7,5" ]; then
    exit 0
else
    echo "Expected move '7,5', got: $OUTPUT"
    exit 1
fi
