#!/bin/bash

AI_EXECUTABLE=$1

OUTPUT=$(echo "BOARD
0,0,1
1,0,1
2,0,1
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "3,0" ]; then
    exit 0
else
    echo "Expected move '3,0', got: $OUTPUT"
    exit 1
fi