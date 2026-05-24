#!/bin/bash

# test/test_cases/test7.sh

AI_EXECUTABLE=$1

# envoie la commande END et vérifie qu'il n'y a pas de réponse
OUTPUT=$(echo "END" | $AI_EXECUTABLE)

# Attendu : rien
if [ -z "$OUTPUT" ]; then
    echo "Output: $OUTPUT"
    exit 0
else
    echo "Invalid Output: $OUTPUT"
    exit 1
fi