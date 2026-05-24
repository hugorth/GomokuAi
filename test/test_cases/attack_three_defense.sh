# test/test_cases/attack_three_defense.sh
#!/bin/bash
AI_EXECUTABLE=$1
# Test attacking while defending against opponent's three in a row
OUTPUT=$(echo "BOARD
7,7,1
8,8,1
5,5,2
6,5,2
7,5,2
DONE" | "$AI_EXECUTABLE")
if [ "$OUTPUT" = "8,5" ] || [ "$OUTPUT" = "4,5" ]; then
    exit 0
else
    echo "Expected move '8,5' or '4,5', got: $OUTPUT"
    exit 1
fi
