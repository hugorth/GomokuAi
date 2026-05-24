##
## EPITECH PROJECT, 2025
## Gomoku
## File description:
## Makefile
##

NAME			=	pbrain-gomoku-ai

DEBUG_NAME		=	target/debug/$(NAME)
RELEASE_NAME	=	target/release/$(NAME)

LINUX_NAME		=	$(NAME)

BIN_DIR			=	bin

CARGO			=	cargo

CARGO_FLAGS		=

TEST_SCRIPT		=	test/test_run.sh
TEST_DIR		=	test/test_cases

all: release

release:
	@echo "=== Construction en mode RELEASE ==="
	$(CARGO) build --release $(CARGO_FLAGS)
	@mkdir -p $(BIN_DIR)
	cp $(RELEASE_NAME) $(LINUX_NAME)
	@echo "Binaire release copié dans $(LINUX_NAME)"

debug:
	@echo "=== Construction en mode DEBUG ==="
	$(CARGO) build $(CARGO_FLAGS)
	@mkdir -p $(BIN_DIR)
	cp $(DEBUG_NAME) $(BIN_DIR)/$(LINUX_NAME)
	@echo "Binaire debug copié dans $(BIN_DIR)/$(LINUX_NAME)"

test: release
	@echo "=== Exécution des tests ==="
	@if [ -f "$(TEST_SCRIPT)" ]; then \
		bash $(TEST_SCRIPT) ./$(LINUX_NAME) $(TEST_DIR); \
	else \
		echo "Le script de test $(TEST_SCRIPT) est introuvable !"; \
		exit 1; \
	fi
	@echo "Tests terminés."

clean:
	@echo "=== Nettoyage des artefacts de compilation ==="
	$(CARGO) clean
	rm -rf $(BIN_DIR)
	@echo "Nettoyage terminé."

fclean: clean

re: fclean all

deps:
	@echo "=== Installation des dépendances ==="
	$(CARGO) build --locked

.PHONY: all release debug test clean fclean re deps