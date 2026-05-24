//
// EPITECH PROJECT, 2025
// minimax.rs
// File description:
// Minimax algorithm implementation for the Gomoku game engine
//

use crate::commands::Game;
use super::{CANDIDATE_RADIUS};

const NEG_INF: i32 = -1_000_000_000;
const POS_INF: i32 =  1_000_000_000;

use super::patterns::{
    is_five_in_a_row,
    evaluate_position,
};
use std::time::{SystemTime, UNIX_EPOCH};

///
/// Computes the best move for the AI player using the minimax algorithm
/// 
/// # Arguments
/// * `game` - Mutable reference to the current game state
/// * `depth` - The maximum depth to search in the game tree
/// 
/// # Returns
/// * A tuple (x, y) representing the coordinates of the best move
///
pub fn compute_minimax_move(game: &mut Game, depth: i32) -> (usize, usize) {
    let candidates = get_candidate_moves(game);
    if candidates.is_empty() {
        return (game.size / 2, game.size / 2);
    }

    let mut best_score = NEG_INF;
    let mut best_move = (usize::MAX, usize::MAX);

    for &(x, y) in &candidates {
        game.board[y][x] = 1;
        let score = minimax(game, depth - 1, NEG_INF, POS_INF, false);
        game.board[y][x] = 0;

        if score > best_score {
            best_score = score;
            best_move = (x, y);
        }
    }
    best_move
}

///
/// Implements the minimax algorithm with alpha-beta pruning
/// 
/// # Arguments
/// * `game` - Mutable reference to the current game state
/// * `depth` - Current depth in the search tree
/// * `alpha` - Alpha value for alpha-beta pruning
/// * `beta` - Beta value for alpha-beta pruning
/// * `maximizing_player` - Boolean indicating if current player is maximizing
/// 
/// # Returns
/// * An integer score representing the evaluated position
///
pub fn minimax(game: &mut Game, depth: i32, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> i32 {
    if let Some(score) = evaluate_terminal_state(game, depth) {
        return score;
    }

    let candidates = get_candidate_moves(game);
    if candidates.is_empty() {
        return evaluate_board_heuristic(game);
    }

    if maximizing_player {
        let mut best_eval = NEG_INF;
        for &(x, y) in &candidates {
            game.board[y][x] = 1;
            let eval = minimax(game, depth - 1, alpha, beta, false);
            game.board[y][x] = 0;
            best_eval = best_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        best_eval
    } else {
        let mut best_eval = POS_INF;
        for &(x, y) in &candidates {
            game.board[y][x] = 2;
            let eval = minimax(game, depth - 1, alpha, beta, true);
            game.board[y][x] = 0;
            best_eval = best_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        best_eval
    }
}

///
/// Generates a list of valid candidate moves near existing pieces
/// 
/// # Arguments
/// * `game` - Reference to the current game state
/// 
/// # Returns
/// * Vector of (x, y) coordinates representing possible moves
///
pub fn get_candidate_moves(game: &Game) -> Vec<(usize, usize)> {
    let mut moves = vec![];
    let mut have_pion = false;
    let mut points = vec![];

    for y in 0..game.size {
        for x in 0..game.size {
            if game.board[y][x] != 0 {
                have_pion = true;
                points.push((x, y));
            }
        }
    }

    if !have_pion {
        moves.push((game.size / 2, game.size / 2));
        return moves;
    }

    let size_i = game.size as isize;
    for &(px, py) in &points {
        let px_i = px as isize;
        let py_i = py as isize;
        for dy in -CANDIDATE_RADIUS..=CANDIDATE_RADIUS {
            for dx in -CANDIDATE_RADIUS..=CANDIDATE_RADIUS {
                let nx = px_i + dx as isize;
                let ny = py_i + dy as isize;
                if nx >= 0 && nx < size_i && ny >= 0 && ny < size_i {
                    let ux = nx as usize;
                    let uy = ny as usize;
                    if game.board[uy][ux] == 0 {
                        moves.push((ux, uy));
                    }
                }
            }
        }
    }

    moves.sort_unstable();
    moves.dedup();
    moves
}

///
/// Evaluates if the current game state is terminal (win/loss/draw) or depth limit reached
/// 
/// # Arguments
/// * `game` - Mutable reference to the current game state
/// * `depth` - Current depth in the search tree
/// 
/// # Returns
/// * Option containing score if terminal state, None otherwise
///
pub fn evaluate_terminal_state(game: &mut Game, depth: i32) -> Option<i32> {
    if is_there_a_winner(game, 1) {
        return Some(100000 + depth);
    }
    if is_there_a_winner(game, 2) {
        return Some(-100000 - depth);
    }
    if board_is_full(game) {
        return Some(0);
    }
    if depth == 0 {
        return Some(evaluate_board_heuristic(game));
    }
    None
}

///
/// Checks if the specified player has won the game
/// 
/// # Arguments
/// * `game` - Reference to the current game state
/// * `player` - Player number (1 ou 2) à vérifier
/// 
/// # Returns
/// * Boolean indiquant si le joueur a gagné
///
pub fn is_there_a_winner(game: &Game, player: u8) -> bool {
    for y in 0..game.size {
        for x in 0..game.size {
            if game.board[y][x] == player {
                if is_five_in_a_row(&game.board, x, y, player) {
                    return true;
                }
            }
        }
    }
    false
}

///
/// Check if the board is full (no empty cells left)
/// 
/// # Arguments
/// * `game` - Reference to the current game state
/// 
/// # Returns
/// * Boolean indicating if the board is full
///
pub fn board_is_full(game: &Game) -> bool {
    for row in &game.board {
        for &cell in row {
            if cell == 0 {
                return false;
            }
        }
    }
    true
}

///
/// Evaluate the current position of the board using a heuristic score
/// 
/// # Arguments
/// * `game` - Reference to the current state of the game
/// 
/// # Returns
/// * An integer score representing position (positive for player 1, negative for player 2)
///
pub fn evaluate_board_heuristic(game: &Game) -> i32 {
    let mut score1 = 0;
    let mut score2 = 0;
    for y in 0..game.size {
        for x in 0..game.size {
            let c = game.board[y][x];
            if c == 1 {
                score1 += evaluate_position(&game.board, x, y, 1);
            } else if c == 2 {
                score2 += evaluate_position(&game.board, x, y, 2);
            }
        }
    }
    score1 - score2
}

///
/// Computes a pseudo-random valid move without using the `rand` dependency.
/// We use the system time to generate an index.
/// 
/// # Arguments
/// * `game` - Reference to the current game state
/// 
/// # Returns
/// * A tuple (x, y) representing the coordinates of the random move
///
pub fn compute_random_move(game: &Game) -> (usize, usize) {
    let cands = get_candidate_moves(game);
    if cands.is_empty() {
        return (game.size / 2, game.size / 2);
    }
    // Generate a pseudo-random index using the system time
    let now = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let nanos = now.as_nanos();
    let idx = (nanos % (cands.len() as u128)) as usize;
    cands[idx]
}
