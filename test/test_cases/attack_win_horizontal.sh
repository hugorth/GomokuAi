# test/test_cases/attack_win_horizontal.sh
#!/bin/bash
AI_EXECUTABLE=$1
# Test winning move with horizontal four in a row
OUTPUT=$(echo "BOARD
5,5,1
6,5,1
7,5,1
8,5,1
DONE" | "$AI_EXECUTABLE")
if [ "$OUTPUT" = "9,5" ]; then
    exit 0
else
    echo "Expected move '9,5', got: $OUTPUT"
    exit 1
fi
