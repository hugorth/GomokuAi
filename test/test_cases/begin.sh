#!/bin/bash

# test/test_cases/test3.sh

AI_EXECUTABLE=$1

# Envoie la commande BEGIN et capture la réponse
OUTPUT=$(echo "BEGIN" | "$AI_EXECUTABLE")

# Attendu : Deux nombres séparés par une virgule, par exemple "10,10"
if [[ "$OUTPUT" =~ ^[0-9]+,[0-9]+$ ]]; then
    echo "Output: $OUTPUT"
    exit 0
else
    echo "Invalid Output: $OUTPUT"
    exit 1
fi
