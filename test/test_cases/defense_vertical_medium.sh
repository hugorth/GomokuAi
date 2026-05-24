#!/bin/bash

AI_EXECUTABLE=$1

OUTPUT=$(echo "BOARD
5,5,1
5,6,1
5,8,1
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "5,7" ]; then
    exit 0
else
    echo "Expected move '5,7', got: $OUTPUT"
    exit 1
fi
