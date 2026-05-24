#!/bin/bash

# test/test_cases/test6.sh

AI_EXECUTABLE=$1

#envoie la commande turn avec des coordonnées
OUTPUT=$(echo "TURN -1;1000" | "$AI_EXECUTABLE")

# Attendu : ERROR Invalid coordinates
EXPECTED="ERROR invalid TURN coords"

if [ "$OUTPUT" == "$EXPECTED" ]; then
    echo "Output: $OUTPUT"
    exit 0
else
    echo "Unexpected Output: $OUTPUT"
    exit 1
fi
