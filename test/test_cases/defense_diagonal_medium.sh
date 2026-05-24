#!/bin/bash

AI_EXECUTABLE=$1

OUTPUT=$(echo "BOARD
5,5,1
6,6,1
8,8,1
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "7,7" ]; then
    exit 0
else
    echo "Expected move '7,7', got: $OUTPUT"
    exit 1
fi
