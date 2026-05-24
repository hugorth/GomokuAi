//
// EPITECH PROJECT, 2025
// main.rs
// File description:
// main function for the Gomoku game engine
//

mod commands;
mod algo;

use std::io::{self, BufRead, Write};
use commands::{Game, parse_command};

/// The main entry point for the Gomoku game engine.
///
/// # Function Operation
/// 1. Creates a new game instance
/// 2. Enters a loop that:
///    - Reads commands from standard input
///    - Processes each command using the parse_command function
///    - Outputs responses to standard output
///    - Continues until an "END" command is received
///
/// # Error Handling
/// - Handles IO errors gracefully by breaking the loop
/// - Skips empty lines in the input
/// - Ensures output is flushed after each response
///
/// # Example
/// The program accepts commands in the format specified by the game protocol:
/// ```text
/// START [size]
/// TURN [x,y]
/// BEGIN
/// BOARD
/// INFO [key] [value]
/// END

fn main() {
    let stdin = io::stdin();
    let mut stdout = std::io::stdout();
    let mut game = Game::new();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l.trim().to_string(),
            Err(_) => break,
        };
        if line.is_empty() {
            continue;
        }

        if let Some(response) = parse_command(&line, &mut game) {
            println!("{}", response);
            stdout.flush().unwrap();
        }

        if line.to_uppercase().starts_with("END") {
            break;
        }
    }
}
