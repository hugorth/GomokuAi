#!/bin/bash

# test/test_run.sh

AI_EXECUTABLE="$1"
TEST_DIR="$2"

# Vérifie si l'exécutable existe
if [ ! -f "$AI_EXECUTABLE" ]; then
    echo "Executable $AI_EXECUTABLE not found! Veuillez d'abord compiler l'IA avec 'make build'."
    exit 1
fi

# Compteur de tests
TEST_NUM=1
PASSED=0
FAILED=0

# Parcourt tous les scripts de test
for test_script in "$TEST_DIR"/*.sh; do
    echo "Running Test #$TEST_NUM: $(basename "$test_script")"

    # Exécute le script et capture la sortie
    bash "$test_script" "$AI_EXECUTABLE"
    RESULT=$?

    if [ $RESULT -eq 0 ]; then
        echo "Test #$TEST_NUM Passed ✅"
        PASSED=$((PASSED + 1))
    else
        echo "Test #$TEST_NUM Failed ❌"
        FAILED=$((FAILED + 1))
    fi

    echo "-----------------------------------"
    TEST_NUM=$((TEST_NUM + 1))
done

# Résumé des résultats
echo "Total Tests: $((PASSED + FAILED))"
echo "Passed: $PASSED ✅"
echo "Failed: $FAILED ❌"

# Sortie avec code d'erreur si des tests ont échoué
if [ "$FAILED" -ne 0 ]; then
    exit 1
else
    exit 0
fi
