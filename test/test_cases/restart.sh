#!/bin/bash
# test/test_cases/restart.sh

AI_EXECUTABLE=$1

# Send all commands in a single stream
OUTPUT=$(printf "START 15\nBEGIN\nRESTART\n" | "$AI_EXECUTABLE")

# Get the last line of output (RESTART response)
LAST_LINE=$(echo "$OUTPUT" | tail -n 1)

if [[ "$LAST_LINE" == "OK" ]]; then
    echo "RESTART test passed"
    exit 0
else
    echo "RESTART test failed: Expected 'OK', got: $LAST_LINE"
    exit 1
fi