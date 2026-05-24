# test/test_cases/attack_win_diagonal.sh
#!/bin/bash
AI_EXECUTABLE=$1
# Test winning move with diagonal four in a row
OUTPUT=$(echo "BOARD
5,5,1
6,6,1
7,7,1
8,8,1
DONE" | "$AI_EXECUTABLE")
if [ "$OUTPUT" = "9,9" ]; then
    exit 0
else
    echo "Expected move '9,9', got: $OUTPUT"
    exit 1
fi
