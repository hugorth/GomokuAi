#!/bin/bash

AI_EXECUTABLE=$1

OUTPUT=$(echo "BOARD
5,5,1
6,5,1
7,5,1
8,5,1
10,10,1
11,10,1
12,10,1
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "4,5" ] || [ "$OUTPUT" = "9,5" ]; then
    exit 0
else
    echo "Expected move '4,5' or '9,5', got: $OUTPUT"
    exit 1
fi