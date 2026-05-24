# Gomoku AI (pbrain-gomoku-ai)

## Description
This project is an Artificial Intelligence for the game of Gomoku, developed as part of an Epitech project (2025). The AI communicates via standard input and output (stdin/stdout) following the standard continuous Gomoku protocol (similar to Pisqpipe).

It evaluates the board state, looks for patterns, and determines the best possible moves using the **Minimax** algorithm.

## Features
- **Rust-powered**: Fast, memory-safe, and highly concurrent.
- **Protocol compatibility**: Handles standard commands (`START`, `TURN`, `BOARD`, `BEGIN`, `INFO`, `END`).
- **AI Engine**: Implements the **Minimax** algorithm paired with pattern recognition to block threats and build winning plays.

## Prerequisites
You need the following installed on your system to build and run this project:
- [Rust & Cargo](https://www.rust-lang.org/tools/install)
- `make` (optional, Makefile provided)

## Compilation
You can compile the binary using the provided `Makefile`:

```bash
make
```

This will run Cargo and generate the executable file named `pbrain-gomoku-ai`. 
Alternatively, you can build a release version using Cargo directly:

```bash
cargo build --release
```

## Usage
Run the binary directly. Since it expects protocol commands via standard input, you can type commands manually or plug it into a Gomoku manager/GUI (such as Piskvork).

```bash
./pbrain-gomoku-ai
```

### Example Communication:
```text
> START 20
< OK
> TURN 10,10
< 10,11
> END
```
*( `>` represents stdin (input), `<` represents stdout (output) )*

## Project Structure
- `src/main.rs` - Entry point and game loop handling I/O.
- `src/commands/` - Command parsing and game state representation.
- `src/algo/` - AI logic including the Minimax algorithm and pattern evaluation.
- `test/` - Contains tests for the engine.

## 🎓 Academic Project
This is an academic project created for EPITECH (2025).
