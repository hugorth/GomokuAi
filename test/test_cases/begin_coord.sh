#!/bin/bash

# test/test_cases/test4.sh

AI_EXECUTABLE=$1

# Envoie la commande BEGIN avec des arguments et capture la réponse
OUTPUT=$(echo "BEGIN 13;7" | "$AI_EXECUTABLE")

# Attendu : ERROR Begin command - No arguments expected.
EXPECTED="ERROR Begin command - No arguments expected."

if [ "$OUTPUT" == "$EXPECTED" ]; then
    echo "Output: $OUTPUT"
    exit 0
else
    echo "Unexpected Output: $OUTPUT"
    exit 1
fi
