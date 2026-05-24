#!/bin/bash

# test/test_cases/test10.sh

AI_EXECUTABLE=$1

# Test diagonal threat scenario
OUTPUT=$(echo "BOARD
5,5,1
5,6,2
6,6,1
6,7,2
7,7,1
DONE" | "$AI_EXECUTABLE")

# Verify that the output is a valid move (two numbers separated by a comma)
if [[ $OUTPUT =~ ^[0-9]+,[0-9]+$ ]]; then
    # You can add more specific validation here if needed
    exit 0
else
    echo "Expected a valid move (x,y), got: $OUTPUT"
    exit 1
fi
