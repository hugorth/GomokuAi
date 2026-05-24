#!/bin/bash
# test/test_cases/attack_moderate2.sh
AI_EXECUTABLE=$1
OUTPUT=$(echo "BOARD
8,8,1
9,8,1
10,8,1
8,9,1
DONE" | "$AI_EXECUTABLE")
if [ "$OUTPUT" = "11,8" ] || [ "$OUTPUT" = "7,8" ]; then
    exit 0
else
    echo "Expected move '11,8' or '7,8', got: $OUTPUT"
    exit 1
fi
