use crate::board::movegen::Move;
use crate::board::piece::{Color, Piece};
use crate::board::position::{CastlingRights, Position};

impl Position {
    pub fn make_move(&self, mv: &Move) -> Position {
        let mut pos = self.clone();
        let color = pos.side_to_move;

        pos.remove_piece(color, mv.piece, mv.from);

        if let Some(cap) = mv.capture {
            let cap_sq = if mv.piece == Piece::Pawn && Some(mv.to) == self.en_passant {
                match color {
                    Color::White => mv.to - 8,
                    Color::Black => mv.to + 8,
                }
            } else {
                mv.to
            };
            pos.remove_piece(color.flip(), cap, cap_sq);
        }

        let placed = mv.promotion.unwrap_or(mv.piece);
        pos.place_piece(color, placed, mv.to);

        if mv.piece == Piece::King {
            let (kside_to, qside_to, rook_from_k, rook_to_k, rook_from_q, rook_to_q) = match color {
                Color::White => (6u8, 2u8, 7u8, 5u8, 0u8, 3u8),
                Color::Black => (62u8, 58u8, 63u8, 61u8, 56u8, 59u8),
            };
            if mv.to == kside_to {
                pos.remove_piece(color, Piece::Rook, rook_from_k);
                pos.place_piece(color, Piece::Rook, rook_to_k);
            } else if mv.to == qside_to {
                pos.remove_piece(color, Piece::Rook, rook_from_q);
                pos.place_piece(color, Piece::Rook, rook_to_q);
            }
            match color {
                Color::White => { pos.castling.remove(CastlingRights::WK); pos.castling.remove(CastlingRights::WQ); }
                Color::Black => { pos.castling.remove(CastlingRights::BK); pos.castling.remove(CastlingRights::BQ); }
            }
        }

        if mv.piece == Piece::Rook {
            match mv.from {
                0  => pos.castling.remove(CastlingRights::WQ),
                7  => pos.castling.remove(CastlingRights::WK),
                56 => pos.castling.remove(CastlingRights::BQ),
                63 => pos.castling.remove(CastlingRights::BK),
                _  => {}
            }
        }

        match mv.to {
            0  => pos.castling.remove(CastlingRights::WQ),
            7  => pos.castling.remove(CastlingRights::WK),
            56 => pos.castling.remove(CastlingRights::BQ),
            63 => pos.castling.remove(CastlingRights::BK),
            _  => {}
        }

        pos.en_passant = if mv.piece == Piece::Pawn && mv.to.abs_diff(mv.from) == 16 {
            Some((mv.from + mv.to) / 2)
        } else {
            None
        };

        pos.halfmove_clock = if mv.piece == Piece::Pawn || mv.capture.is_some() { 0 } else { pos.halfmove_clock + 1 };
        if color == Color::Black { pos.fullmove_number += 1; }
        pos.side_to_move = color.flip();
        pos
    }
}