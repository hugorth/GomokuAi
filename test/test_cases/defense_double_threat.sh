#!/bin/bash

AI_EXECUTABLE=$1

# Test defense against double threat
# On simule deux menaces adverses (joueur 2)
OUTPUT=$(echo "BOARD
5,5,2
6,6,2
7,7,2
5,7,2
6,7,2
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "8,8" ] || [ "$OUTPUT" = "7,7" ]; then
    exit 0
else
    echo "Expected move '8,8' or '7,7', got: $OUTPUT"
    exit 1
fi
