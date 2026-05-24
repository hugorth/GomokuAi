#!/bin/bash
# test/test_cases/attack_easy1.sh
AI_EXECUTABLE=$1
OUTPUT=$(echo "BOARD
7,7,1
8,7,1
9,7,1
DONE" | "$AI_EXECUTABLE")
if [ "$OUTPUT" = "10,7" ] || [ "$OUTPUT" = "6,7" ]; then
    exit 0
else
    echo "Expected move '10,7' or '6,7', got: $OUTPUT"
    exit 1
fi
