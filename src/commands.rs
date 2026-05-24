//
// EPITECH PROJECT, 2025
// commands.rs
// File description:
// communication protocol between the game engine and the AI
//

use crate::algo::{
    compute_smart_move,
};

/// Represents the game state and board configuration
pub struct Game {
    /// Size of the game board (size x size)
    pub size: usize,         
    /// 2D vector representing the game board where:
    /// - 0: Empty cell
    /// - 1: AI player
    /// - 2: Opponent
    pub board: Vec<Vec<u8>>,
    /// Flag indicating if the board is currently being set up
    pub in_board_setup: bool,
}

impl Game {
    /// Creates a new instance of the game with default values
    /// 
    /// # Returns
    /// A new Game instance with empty board and size 0
    pub fn new() -> Self {
        Self {
            size: 0,
            board: Vec::new(),
            in_board_setup: false,
        }
    }

    
    /// Initializes the game board with the specified size
    /// 
    /// # Arguments
    /// * `size` - The size of the board (size x size)
    /// 
    /// # Returns
    /// * `Ok(())` if initialization successful
    /// * `Err(String)` if size is invalid (< 1 or > 100)
    pub fn init(&mut self, size: usize) -> Result<(), String> {
        if size < 1 || size > 100 {
            return Err(format!("Unsupported board size: {size}"));
        }
        self.size = size;
        self.board = vec![vec![0; size]; size];
        Ok(())
    }

    /// Handles opponent's turn and computes AI's response move
    /// 
    /// # Arguments
    /// * `x` - X coordinate of opponent's move
    /// * `y` - Y coordinate of opponent's move
    /// 
    /// # Returns
    /// String containing AI's move coordinates in format "x,y"
    pub fn handle_turn(&mut self, x: usize, y: usize) -> String {
        if self.size == 0 {
            //eprintln!("WARNING: Received TURN without START or BOARD -> defaulting to 20x20");
            self.size = 20;
            self.board = vec![vec![0; 20]; 20];
        }

        if x < self.size && y < self.size {
            self.board[y][x] = 2;
        } else {
            eprintln!("WARNING: Opponent's move ({}, {}) is out of bounds.", x, y);
        }

        let (mx, my) = compute_smart_move(self);
        if mx < self.size && my < self.size {
            self.board[my][mx] = 1;
        } else {
            eprintln!("WARNING: AI's move ({}, {}) is out of bounds.", mx, my);
        }

        format!("{},{}", mx, my)
    }    

    /// Handles the BEGIN command when AI plays first
    /// 
    /// # Returns
    /// String containing AI's first move coordinates in format "x,y"
    pub fn handle_begin(&mut self) -> String {
        if self.size == 0 {
            //eprintln!("WARNING: Received BEGIN without START -> defaulting to 20x20");
            self.size = 20;
            self.board = vec![vec![0; 20]; 20];
        }
        let (mx, my) = compute_smart_move(self);
        if mx < self.size && my < self.size {
            self.board[my][mx] = 1;
        } else {
            eprintln!("WARNING: AI's first move ({}, {}) is out of bounds.", mx, my);
        }
        format!("{},{}", mx, my)
    }    

    /// Adds a move to the board during board setup phase
    /// 
    /// # Arguments
    /// * `x` - X coordinate of the move
    /// * `y` - Y coordinate of the move
    /// * `player` - Player number (1 for AI, 2 for opponent)
    pub fn add_board_move(&mut self, x: usize, y: usize, player: u8) {
        if self.size == 0 {
            self.size = 20;
            self.board = vec![vec![0; 20]; 20];
        }

        if x < self.size && y < self.size {
            self.board[y][x] = player;
        } else {
            eprintln!("WARNING: BOARD move ({}, {}) is out of bounds.", x, y);
        }
    }

     /// Completes the board setup phase and computes AI's next move
    /// 
    /// # Returns
    /// String containing AI's move coordinates in format "x,y"
    pub fn finish_board_setup(&mut self) -> String {
        let (mx, my) = compute_smart_move(self);
        if mx < self.size && my < self.size {
            self.board[my][mx] = 1;
        } else {
            eprintln!("WARNING: AI's move after BOARD ({}, {}) is out of bounds.", mx, my);
        }
        format!("{},{}", mx, my)
    }

    pub fn restart(&mut self) -> Result<(), String> {
        if self.size == 0 {
            return Err("Cannot restart: board was never initialized".to_string());
        }
        self.board = vec![vec![0; self.size]; self.size];
        self.in_board_setup = false;
        Ok(())
    }
}

/// Parses and processes game commands
/// 
/// # Arguments
/// * `line` - The command line to parse
/// * `game` - Mutable reference to the game state
/// 
/// # Returns
/// * `Some(String)` containing the response to the command
/// * `None` if no response is needed
/// 
/// # Supported Commands
/// - START [size]: Initializes the game board
/// - BEGIN: AI plays first move
/// - TURN [x,y]: Processes opponent's move
/// - BOARD: Enters board setup mode
/// - DONE: Completes board setup
/// - INFO: Ignored
/// - END: Terminates the game
/// - ABOUT: Returns game information
pub fn parse_command(line: &str, game: &mut Game) -> Option<String> {
    let mut parts = line.split_whitespace();
    let cmd = parts.next().unwrap_or("").to_uppercase();
    let args = parts.collect::<Vec<_>>().join(" ");

    match cmd.as_str() {
        "START" => {
            if let Ok(sz) = args.parse::<usize>() {
                match game.init(sz) {
                    Ok(_) => Some("OK".to_string()),
                    Err(e) => Some(format!("ERROR {e}")),
                }
            } else {
                Some("ERROR invalid START parameter".to_string())
            }
        }

        "RESTART" => {
            match game.restart() {
                Ok(_) => Some("OK".to_string()),
                Err(e) => Some(format!("ERROR {e}")),
            }
        }

        "BEGIN" => {
            if args.trim().is_empty() {
                Some(game.handle_begin())
            } else {
                Some("ERROR Begin command - No arguments expected.".to_string())
            }
        }

        "TURN" => {
            if let Some((x, y)) = parse_coords(&args) {
                Some(game.handle_turn(x, y))
            } else {
                Some("ERROR invalid TURN coords".to_string())
            }
        }

        "BOARD" => {
            game.in_board_setup = true;

            if game.size == 0 {
                game.size = 20;
                game.board = vec![vec![0; 20]; 20];
            }
            None
        }

        "DONE" => {
            if game.in_board_setup {
                game.in_board_setup = false;
                Some(game.finish_board_setup())
            } else {
                Some("ERROR no BOARD in progress".to_string())
            }
        }

        "INFO" => {
            None
        }

        "END" => {
            None
        }

        "ABOUT" => {
            Some(r#"name="RustGomoku", version="0.1", author="PaulHernandez", country="FR""#.to_string())
        }

        _ => {
            if game.in_board_setup {
                if let Some((x, y, f)) = parse_coords_field(line) {
                    game.add_board_move(x, y, f);
                    None
                } else {
                    Some(format!("UNKNOWN {line}"))
                }
            } else {
                Some(format!("UNKNOWN {line}"))
            }
        }
    }
}

/// Parses coordinate pairs from a string
/// 
/// # Arguments
/// * `s` - String containing coordinates in format "x,y"
/// 
/// # Returns
/// * `Some((x, y))` if parsing successful
/// * `None` if parsing fails
fn parse_coords(s: &str) -> Option<(usize, usize)> {
    let mut split = s.split(',');
    let x_str = split.next()?;
    let y_str = split.next()?;
    let x = x_str.trim().parse().ok()?;
    let y = y_str.trim().parse().ok()?;
    Some((x, y))
}

/// Parses coordinates and field value from a string
/// 
/// # Arguments
/// * `s` - String containing coordinates and field in format "x,y,f"
/// 
/// # Returns
/// * `Some((x, y, f))` if parsing successful
/// * `None` if parsing fails
/// 
/// # Field Values
/// - 1: AI player
/// - 2: Opponent
/// - 3: Winning line or forbidden according to Renju rules
fn parse_coords_field(s: &str) -> Option<(usize, usize, u8)> {
    let mut split = s.split(',');
    let x_str = split.next()?;
    let y_str = split.next()?;
    let f_str = split.next()?;
    let x = x_str.trim().parse().ok()?;
    let y = y_str.trim().parse().ok()?;
    let f = f_str.trim().parse().ok()?;
    Some((x, y, f))
}
