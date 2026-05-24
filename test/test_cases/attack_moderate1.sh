#!/bin/bash
# test/test_cases/attack_moderate1.sh
AI_EXECUTABLE=$1
OUTPUT=$(echo "BOARD
7,7,1
8,8,1
9,9,1
10,10,1
DONE" | "$AI_EXECUTABLE")
if [ "$OUTPUT" = "11,11" ] || [ "$OUTPUT" = "6,6" ]; then
    exit 0
else
    echo "Expected move '11,11' or '6,6', got: $OUTPUT"
    exit 1
fi
