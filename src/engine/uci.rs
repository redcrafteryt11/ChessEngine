use std::io::{self, BufRead};
use crate::board::{position::Position, movegen::Move};
use super::search::search;

fn move_to_uci(mv: &Move) -> String {
    let from = format!("{}{}", (b'a' + mv.from % 8) as char, (b'1' + mv.from / 8) as char);
    let to   = format!("{}{}", (b'a' + mv.to   % 8) as char, (b'1' + mv.to   / 8) as char);
    let promo = mv.promotion.map(|p| match p {
        crate::board::piece::Piece::Queen  => "q",
        crate::board::piece::Piece::Rook   => "r",
        crate::board::piece::Piece::Bishop => "b",
        crate::board::piece::Piece::Knight => "n",
        _ => "",
    }).unwrap_or("");
    format!("{from}{to}{promo}")
}

fn parse_move(pos: &Position, token: &str) -> Option<Move> {
    use crate::board::movegen::{generate_pawn_moves, generate_knight_moves, generate_sliding_moves, generate_king_moves, is_in_check};
    let mut moves = Vec::new();
    generate_pawn_moves(pos, &mut moves);
    generate_knight_moves(pos, &mut moves);
    generate_sliding_moves(pos, &mut moves);
    generate_king_moves(pos, &mut moves);
    moves.retain(|mv| !is_in_check(&pos.make_move(mv), pos.side_to_move));
    moves.into_iter().find(|mv| move_to_uci(mv) == token)
}

pub fn run() {
    let stdin = io::stdin();
    let mut pos = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
        .expect("invalid startpos");

    for line in stdin.lock().lines() {
        let line = line.expect("stdin error");
        let mut tokens = line.split_whitespace();
        match tokens.next() {
            Some("uci") => {
                println!("id name ChessEngine");
                println!("id author Nicolas");
                println!("uciok");
            }
            Some("isready") => println!("readyok"),
            Some("ucinewgame") => {
                pos = Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                    .expect("invalid startpos");
            }
            Some("position") => {
                let rest: Vec<&str> = tokens.collect();
                let moves_idx = rest.iter().position(|&t| t == "moves");
                let fen = if rest.first() == Some(&"startpos") {
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
                } else {
                    let end = moves_idx.unwrap_or(rest.len());
                    rest[1..end].join(" ")
                };
                pos = Position::from_fen(&fen).expect("invalid fen");
                if let Some(idx) = moves_idx {
                    for mv_str in &rest[idx + 1..] {
                        if let Some(mv) = parse_move(&pos, mv_str) {
                            pos = pos.make_move(&mv);
                        }
                    }
                }
            }
            Some("go") => {
                let tokens: Vec<&str> = tokens.collect();
                let depth = tokens.iter().position(|&t| t == "depth")
                    .and_then(|i| tokens.get(i + 1))
                    .and_then(|d| d.parse().ok())
                    .unwrap_or(5);
                if let Some(result) = search(&pos, depth) {
                    println!("info depth {} score cp {}", result.depth, result.score);
                    println!("bestmove {}", move_to_uci(&result.best_move));
                } else {
                    println!("bestmove 0000");
                }
            }
            Some("quit") => break,
            _ => {}
        }
    }
}