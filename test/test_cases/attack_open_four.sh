# test/test_cases/attack_open_four.sh
#!/bin/bash
AI_EXECUTABLE=$1
# Test creating an open four
OUTPUT=$(echo "BOARD
7,7,1
8,7,1
9,7,1
10,7,1
DONE" | "$AI_EXECUTABLE")
if [ "$OUTPUT" = "6,7" ] || [ "$OUTPUT" = "11,7" ]; then
    exit 0
else
    echo "Expected move '6,7' or '11,7', got: $OUTPUT"
    exit 1
fi
