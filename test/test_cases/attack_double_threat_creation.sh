# test/test_cases/attack_double_threat_creation.sh
#!/bin/bash
AI_EXECUTABLE=$1
# Test creating a double threat that leads to win
OUTPUT=$(echo "BOARD
5,5,1
6,6,1
7,7,1
5,7,1
6,7,1
DONE" | "$AI_EXECUTABLE")
if [ "$OUTPUT" = "8,8" ] || [ "$OUTPUT" = "7,8" ]; then
    exit 0
else
    echo "Expected move '8,8' or '7,8', got: $OUTPUT"
    exit 1
fi
