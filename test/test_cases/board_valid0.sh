#!/bin/bash

# test/test_cases/test9.sh

AI_EXECUTABLE=$1

# Test empty board scenario
OUTPUT=$(echo "BOARD
DONE" | "$AI_EXECUTABLE")

# Verify that the output is a valid move (two numbers separated by a comma)
if [[ $OUTPUT =~ ^[0-9]+,[0-9]+$ ]]; then
    exit 0
else
    echo "Expected a valid move (x,y), got: $OUTPUT"
    exit 1
fi
