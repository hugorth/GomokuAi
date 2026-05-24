#!/bin/bash

AI_EXECUTABLE=$1

OUTPUT=$(echo "BOARD
10,10,1
11,10,1
12,10,1
13,10,1
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "9,10" ] || [ "$OUTPUT" = "14,10" ]; then
    exit 0
else
    echo "Expected move '9,10' or '14,10', got: $OUTPUT"
    exit 1
fi