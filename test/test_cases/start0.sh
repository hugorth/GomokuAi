#!/bin/bash

# test/test_cases/test2.sh

AI_EXECUTABLE=$1

# Envoie la commande START 0 et capture la réponse
OUTPUT=$(echo "START 0" | "$AI_EXECUTABLE")

# Attendu : ERROR [message]
if [[ "$OUTPUT" =~ ^ERROR.*$ ]]; then
    echo "Output: $OUTPUT"
    exit 0
else
    echo "Unexpected Output: $OUTPUT"
    exit 1
fi
