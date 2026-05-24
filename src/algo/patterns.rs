//
// EPITECH PROJECT, 2025
// patterns.rs
// File description:
// patterns module for the Gomoku game engine
//

use crate::commands::Game;

///
/// Searches for an immediate winning move that creates five in a row
///
/// # Arguments
/// * `game` - Mutable reference to the current game state
/// * `player` - Player number (1 or 2) to find winning move for
///
/// # Returns
/// * Option containing (x, y) coordinates of winning move if found, None otherwise
///
pub fn find_immediate_five(game: &mut Game, player: u8) -> Option<(usize, usize)> {
    for y in (0..game.size).rev() {
        for x in (0..game.size).rev() {
            if game.board[y][x] == 0 {
                game.board[y][x] = player;
                if is_five_in_a_row(&game.board, x, y, player) {
                    game.board[y][x] = 0;
                    return Some((x, y));
                }
                game.board[y][x] = 0;
            }
        }
    }
    None
}

///
/// Identifies the most threatening pattern from the opponent that needs to be blocked
///
/// # Arguments
/// * `game` - Mutable reference to the current game state
/// * `opp_player` - Opponent's player number to analyze threats for
///
/// # Returns
/// * Option containing (x, y) coordinates of the blocking move if threat score > 500, None otherwise
///
pub fn find_most_dangerous_pattern(game: &mut Game, opp_player: u8) -> Option<(usize, usize)> {
    let mut best_score = 0;
    let mut best_case: Option<(usize, usize)> = None;

    for y in (0..game.size).rev() {
        for x in (0..game.size).rev() {
            if game.board[y][x] == 0 {
                game.board[y][x] = opp_player;
                let pat_score = evaluate_local_pattern(&game.board, x, y, opp_player);
                game.board[y][x] = 0;

                if pat_score > best_score {
                    best_score = pat_score;
                    best_case = Some((x, y));
                }
            }
        }
    }

    if let Some(case) = best_case {
        if best_score > 500 {
            return Some(case);
        }
    }
    None
}

///
/// Finds the best offensive pattern to build upon
///
/// # Arguments
/// * `game` - Mutable reference to the current game state
/// * `player` - Player number to find offensive moves for
///
/// # Returns
/// * Option containing (x, y) coordinates of the best offensive move if pattern score > 300, None otherwise
///
pub fn find_best_offensive_pattern(game: &mut Game, player: u8) -> Option<(usize, usize)> {
    let mut best_score = 0;
    let mut best_case: Option<(usize, usize)> = None;

    for y in (0..game.size).rev() {
        for x in (0..game.size).rev() {
            if game.board[y][x] == 0 {
                game.board[y][x] = player;
                let pat_score = evaluate_local_pattern(&game.board, x, y, player);
                game.board[y][x] = 0;

                if pat_score > best_score {
                    best_score = pat_score;
                    best_case = Some((x, y));
                }
            }
        }
    }

    if let Some(case) = best_case {
        if best_score > 300 {
            return Some(case);
        }
    }
    None
}

///
/// Evaluates the strength of a pattern at a specific position
///
/// # Arguments
/// * `board` - Reference to the game board
/// * `x` - X coordinate of the position to evaluate
/// * `y` - Y coordinate of the position to evaluate
/// * `player` - Player number whose pattern to evaluate
///
/// # Returns
/// * Integer score representing the pattern strength
///
pub fn evaluate_local_pattern(board: &Vec<Vec<u8>>, x: usize, y: usize, player: u8) -> i32 {
    let directions = [(1,0), (0,1), (1,1), (1,-1)];
    let mut best_dir_score = 0;

    for &(dx, dy) in &directions {
        let len = 1
          + count_along(board, x, y, dx, dy, player)
          + count_along(board, x, y, -dx, -dy, player);
        let (open_ends, blocked) = count_open_ends(board, x, y, dx, dy, player);
        let dir_score = pattern_score(len, open_ends, blocked);
        if dir_score > best_dir_score {
            best_dir_score = dir_score;
        }
    }
    best_dir_score
}

///
/// Counts consecutive pieces in a specific direction
///
/// # Arguments
/// * `board` - Reference to the game board
/// * `x` - Starting X coordinate
/// * `y` - Starting Y coordinate
/// * `dx` - X direction (-1, 0, or 1)
/// * `dy` - Y direction (-1, 0, or 1)
/// * `player` - Player number whose pieces to count
///
/// # Returns
/// * Number of consecutive pieces found in the specified direction
///
pub fn count_along(board: &Vec<Vec<u8>>, x: usize, y: usize, dx: isize, dy: isize, player: u8) -> u32 {
    let mut c = 0;
    let mut cx = x as isize + dx;
    let mut cy = y as isize + dy;
    let size = board.len() as isize;

    while cx >= 0 && cx < size && cy >= 0 && cy < size {
        if board[cy as usize][cx as usize] == player {
            c += 1;
            cx += dx;
            cy += dy;
        } else {
            break;
        }
    }
    c
}

///
/// Counts the number of open ends and checks if a pattern is blocked
///
/// # Arguments
/// * `board` - Reference to the game board
/// * `x` - X coordinate of the position
/// * `y` - Y coordinate of the position
/// * `dx` - X direction (-1, 0, or 1)
/// * `dy` - Y direction (-1, 0, or 1)
/// * `player` - Player number whose pattern to analyze
///
/// # Returns
/// * Tuple containing (number of open ends, whether pattern is blocked)
///
pub fn count_open_ends(
    board: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    player: u8
) -> (u8, bool) {
    let ( _cx1, _cy1, (open1, blocked1)) = check_side(board, x, y, dx, dy, player);
    let ( _cx2, _cy2, (open2, blocked2)) = check_side(board, x, y, -dx, -dy, player);

    let open_ends = open1 + open2;
    let is_blocked = blocked1 || blocked2;
    (open_ends, is_blocked)
}

///
/// Checks one side of a pattern for openness and blocking
///
/// # Arguments
/// * `board` - Reference to the game board
/// * `x` - Starting X coordinate
/// * `y` - Starting Y coordinate
/// * `dx` - X direction (-1, 0, or 1)
/// * `dy` - Y direction (-1, 0, or 1)
/// * `player` - Player number whose pattern to check
///
/// # Returns
/// * Tuple containing (final x position, final y position, (open ends count, is blocked))
///
pub fn check_side(
    board: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    player: u8
) -> (isize, isize, (u8, bool)) {
    let size = board.len() as isize;
    let mut cx = x as isize + dx;
    let mut cy = y as isize + dy;

    while cx >= 0 && cx < size && cy >= 0 && cy < size {
        if board[cy as usize][cx as usize] == player {
            cx += dx;
            cy += dy;
        } else {
            break;
        }
    }

    if cx < 0 || cx >= size || cy < 0 || cy >= size {
        return (cx, cy, (0, true));
    }
    let c = board[cy as usize][cx as usize];
    if c == 0 {
        (cx, cy, (1, false))
    } else if c != player {
        (cx, cy, (0, true))
    } else {
        (cx, cy, (0, false))
    }
}

///
/// Calculates a score for a pattern based on its length and openness
///
/// # Arguments
/// * `len` - Length of the pattern
/// * `open_ends` - Number of open ends (0-2)
/// * `blocked` - Whether the pattern is blocked
///
/// # Returns
/// * Integer score based on pattern characteristics:
/// * 999999 for five or more in a row
/// * Higher scores for longer patterns with more open ends
/// * Lower scores for blocked or shorter patterns
///
pub fn pattern_score(len: u32, open_ends: u8, blocked: bool) -> i32 {
    if len >= 5 {
        return 999999;
    }

    match len {
        4 => {
            if !blocked && open_ends == 2 {
                12000
            } else if !blocked && open_ends == 1 {
                6000
            } else {
                2000
            }
        }
        3 => {
            if !blocked && open_ends == 2 {
                3000
            } else if !blocked && open_ends == 1 {
                1000
            } else {
                500
            }
        }
        2 => {
            if !blocked && open_ends == 2 {
                300
            } else if !blocked && open_ends == 1 {
                100
            } else {
                50
            }
        }
        1 => {
            10
        }
        _ => 0,
    }
}

///
/// Checks if there are five or more pieces in a row at a position
///
/// # Arguments
/// * `board` - Reference to the game board
/// * `x` - X coordinate to check
/// * `y` - Y coordinate to check
/// * `player` - Player number whose pieces to check
///
/// # Returns
/// * Boolean indicating if five or more pieces are connected
///
pub fn is_five_in_a_row(board: &Vec<Vec<u8>>, x: usize, y: usize, player: u8) -> bool {
    let directions = [(1,0), (0,1), (1,1), (1,-1)];
    for &(dx, dy) in &directions {
        let count = 1
            + count_along(board, x, y, dx, dy, player)
            + count_along(board, x, y, -dx, -dy, player);
        if count >= 5 {
            return true;
        }
    }
    false
}

///
/// Evaluates the strength of a position based on the number of connected pieces
///
/// # Arguments
/// * `board` - Reference to the game board
/// * `x` - X coordinate to evaluate
/// * `y` - Y coordinate to evaluate
/// * `player` - Player number whose position to evaluate
///
/// # Returns
/// * Integer score based on the number of connected pieces in all directions:
/// * 10 points for 2 pieces
/// * 50 points for 3 pieces
/// * 500 points for 4 pieces
/// * 50000 points for 5 pieces
///
pub fn evaluate_position(board: &Vec<Vec<u8>>, x: usize, y: usize, player: u8) -> i32 {
    let directions = [(1,0), (0,1), (1,1), (1,-1)];
    let mut total = 0;
    for &(dx, dy) in &directions {
        let c = 1
            + count_along(board, x, y, dx, dy, player)
            + count_along(board, x, y, -dx, -dy, player);
        match c {
            2 => total += 10,
            3 => total += 50,
            4 => total += 500,
            5 => total += 50000,
            _ => {}
        }
    }
    total
}
