#!/bin/bash
# test/test_cases/attack_easy2.sh
AI_EXECUTABLE=$1
OUTPUT=$(echo "BOARD
7,7,1
7,8,1
7,9,1
DONE" | "$AI_EXECUTABLE")
if [ "$OUTPUT" = "7,10" ] || [ "$OUTPUT" = "7,6" ]; then
    exit 0
else
    echo "Expected move '7,10' or '7,6', got: $OUTPUT"
    exit 1
fi
