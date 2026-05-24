//
// EPITECH PROJECT, 2025
// mod.rs
// File description:
// Algorithm module containing pattern recognition and move computation for Gomoku AI
//

mod patterns;
mod minimax;

use patterns::{
    find_immediate_five,
    find_most_dangerous_pattern,
    find_best_offensive_pattern,
};
use minimax::{
    compute_minimax_move,
    compute_random_move,
};

use crate::commands::Game;

///
/// Maximum depth for the minimax algorithm search
/// Increasing this value will make the AI look further ahead but will increase computation time
/// 
const MAX_DEPTH: i32 = 3;

///
/// Radius around existing pieces to consider for candidate moves
/// Defines how far from existing pieces the AI will consider placing new moves
///
pub const CANDIDATE_RADIUS: i32 = 2;

///
/// Computes the optimal move for the AI player using a combination of pattern recognition
/// and minimax algorithm. The function follows this priority:
/// 1. Winning move (immediate five in a row)
/// 2. Blocking opponent's winning move
/// 3. Defensive moves against dangerous patterns
/// 4. Offensive pattern-based moves
/// 5. Strategic move using minimax
/// 6. Random move as fallback
///
/// # Arguments
/// * `game` - Mutable reference to the current game state
///
/// # Returns
/// * A tuple (x, y) representing the coordinates of the chosen move
///
pub fn compute_smart_move(game: &mut Game) -> (usize, usize) {
    if let Some((x, y)) = find_immediate_five(game, 1) {
        return (x, y);
    }
    if let Some((x, y)) = find_immediate_five(game, 2) {
        return (x, y); 
    }

    if let Some((x, y)) = find_most_dangerous_pattern(game, 2) {
        return (x, y);
    }

    if let Some((x, y)) = find_best_offensive_pattern(game, 1) {
        return (x, y); 
    }

    let (mx, my) = compute_minimax_move(game, MAX_DEPTH);
    if mx == usize::MAX {
        return compute_random_move(game);
    }
    (mx, my)
}
