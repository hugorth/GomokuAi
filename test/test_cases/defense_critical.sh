#!/bin/bash

AI_EXECUTABLE=$1

# Test critical horizontal defense
# Ici, on simule que l'adversaire (joueur 2) a déjà posé ses pions
OUTPUT=$(echo "BOARD
10,10,2
11,10,2
12,10,2
13,10,2
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "14,10" ]; then
    exit 0
else
    echo "Expected move '14,10', got: $OUTPUT"
    exit 1
fi
