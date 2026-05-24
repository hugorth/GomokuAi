#!/bin/bash

AI_EXECUTABLE=$1

OUTPUT=$(echo "BOARD
5,5,1
6,6,1
7,7,1
8,8,1
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "4,4" ] || [ "$OUTPUT" = "9,9" ]; then
    exit 0
else
    echo "Expected move '4,4' or '9,9', got: $OUTPUT"
    exit 1
fi
