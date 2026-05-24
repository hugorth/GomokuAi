#!/bin/bash

# test/test_cases/test1.sh

AI_EXECUTABLE=$1

# Envoie la commande START 20 et capture la réponse
OUTPUT=$(echo "START 20" | "$AI_EXECUTABLE")

# Attendu : OK
EXPECTED="OK"

if [ "$OUTPUT" == "$EXPECTED" ]; then
    echo "Output: $OUTPUT"
    exit 0
else
    echo "Invalid Output: $OUTPUT"
    exit 1
fi
