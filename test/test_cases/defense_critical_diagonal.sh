#!/bin/bash

AI_EXECUTABLE=$1

# Test critical diagonal defense
# Simuler que l'adversaire (joueur 2) a posé ses pions en diagonale
OUTPUT=$(echo "BOARD
5,5,2
6,6,2
7,7,2
8,8,2
DONE" | "$AI_EXECUTABLE")

if [ "$OUTPUT" = "9,9" ]; then
    exit 0
else
    echo "Expected move '9,9', got: $OUTPUT"
    exit 1
fi
