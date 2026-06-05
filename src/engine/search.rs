use crate::board::{position::Position, movegen::{generate_pawn_moves, generate_knight_moves, generate_sliding_moves, generate_king_moves, is_in_check, Move}};
use super::eval::evaluate;

const INF: i32 = 1_000_000;

fn generate_legal_moves(pos: &Position) -> Vec<Move> {
    let mut moves = Vec::new();
    generate_pawn_moves(pos, &mut moves);
    generate_knight_moves(pos, &mut moves);
    generate_sliding_moves(pos, &mut moves);
    generate_king_moves(pos, &mut moves);
    moves.retain(|mv| !is_in_check(&pos.make_move(mv), pos.side_to_move));
    moves
}

fn score_move(mv: &Move) -> i32 {
    match mv.capture {
        Some(cap) => 10 * cap as i32 - mv.piece as i32,
        None => 0,
    }
}

fn alpha_beta(pos: &Position, depth: u32, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 { return evaluate(pos); }

    let mut moves = generate_legal_moves(pos);
    if moves.is_empty() {
        return if is_in_check(pos, pos.side_to_move) { -INF } else { 0 };
    }

    moves.sort_unstable_by_key(|mv| -score_move(mv));

    for mv in &moves {
        let score = -alpha_beta(&pos.make_move(mv), depth - 1, -beta, -alpha);
        if score >= beta { return beta; }
        if score > alpha { alpha = score; }
    }
    alpha
}

pub struct SearchResult {
    pub best_move: Move,
    pub score: i32,
    pub depth: u32,
}

pub fn search(pos: &Position, max_depth: u32) -> Option<SearchResult> {
    let mut best: Option<SearchResult> = None;

    for depth in 1..=max_depth {
        let mut moves = generate_legal_moves(pos);
        if moves.is_empty() { return None; }

        moves.sort_unstable_by_key(|mv| -score_move(mv));

        let mut alpha = -INF;
        let mut best_move = moves[0];

        for mv in &moves {
            let score = -alpha_beta(&pos.make_move(mv), depth - 1, -INF, -alpha);
            if score > alpha {
                alpha = score;
                best_move = *mv;
            }
        }

        best = Some(SearchResult { best_move, score: alpha, depth });
    }
    best
}