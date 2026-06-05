use super::{bitboard::Bitboard, piece::{Color, Piece}, position::{Position, CastlingRights, }};

static KNIGHT_ATTACKS: std::sync::OnceLock<[Bitboard; 64]> = std::sync::OnceLock::new();

fn knight_attacks() -> &'static [Bitboard; 64] {
    KNIGHT_ATTACKS.get_or_init(|| {
        let mut table = [Bitboard::EMPTY; 64];
        for sq in 0u8..64 {
            let bb = Bitboard::from_square(sq);
            let attacks =
                ((bb & !Bitboard::FILE_A & !Bitboard::FILE_B) << 6)  |
                    ((bb & !Bitboard::FILE_G & !Bitboard::FILE_H) << 10) |
                    ((bb & !Bitboard::FILE_A)                     << 15) |
                    ((bb & !Bitboard::FILE_H)                     << 17) |
                    ((bb & !Bitboard::FILE_G & !Bitboard::FILE_H) >> 6)  |
                    ((bb & !Bitboard::FILE_A & !Bitboard::FILE_B) >> 10) |
                    ((bb & !Bitboard::FILE_H)                     >> 15) |
                    ((bb & !Bitboard::FILE_A)                     >> 17);
            table[sq as usize] = attacks;
        }
        table
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub piece: Piece,
    pub capture: Option<Piece>,
    pub promotion: Option<Piece>,
}

impl Move {
    pub fn new(from: u8, to: u8, piece: Piece, capture: Option<Piece>) -> Self {
        Self { from, to, piece, capture, promotion: None }
    }
}

pub fn generate_knight_moves(pos: &Position, moves: &mut Vec<Move>) {
    let color = pos.side_to_move;
    let own = pos.color_bb(color);
    let opp = pos.color_bb(color.flip());
    let knights = pos.piece_bb(color, Piece::Knight);
    for from in knights {
        let attacks = knight_attacks()[from as usize] & !own;
        for to in attacks {
            let capture = if opp.contains(to) { pos.piece_at(to).map(|(_, p)| p) } else { None };
            moves.push(Move::new(from, to, Piece::Knight, capture));
        }
    }
}

pub fn generate_pawn_moves(pos: &Position, moves: &mut Vec<Move>) {
    generate_pawn_moves_for(pos, moves, pos.side_to_move);
}

fn generate_pawn_moves_for(pos: &Position, moves: &mut Vec<Move>, color: Color) {
    let pawns = pos.piece_bb(color, Piece::Pawn);
    let occ = pos.occupancy();
    let opp = pos.color_bb(color.flip());

    let (single, double_rank, promo_rank, dir): (fn(Bitboard) -> Bitboard, Bitboard, Bitboard, i8) =
        match color {
            Color::White => (Bitboard::shift_north, Bitboard::RANK_3, Bitboard::RANK_8, 8),
            Color::Black => (Bitboard::shift_south, Bitboard::RANK_6, Bitboard::RANK_1, -8),
        };

    let single_push = single(pawns) & !occ;
    let double_push = single(single_push & double_rank) & !occ;

    for to in single_push {
        let from = (to as i8 - dir) as u8;
        if Bitboard::from_square(to) & promo_rank != Bitboard::EMPTY {
            for promo in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                moves.push(Move { from, to, piece: Piece::Pawn, capture: None, promotion: Some(promo) });
            }
        } else {
            moves.push(Move { from, to, piece: Piece::Pawn, capture: None, promotion: None });
        }
    }

    for to in double_push {
        let from = (to as i8 - dir * 2) as u8;
        moves.push(Move { from, to, piece: Piece::Pawn, capture: None, promotion: None });
    }

    let (capt_left, capt_right): (fn(Bitboard) -> Bitboard, fn(Bitboard) -> Bitboard) = match color {
        Color::White => (
            |bb| Bitboard::shift_north(Bitboard::shift_west(bb)),
            |bb| Bitboard::shift_north(Bitboard::shift_east(bb)),
        ),
        Color::Black => (
            |bb| Bitboard::shift_south(Bitboard::shift_west(bb)),
            |bb| Bitboard::shift_south(Bitboard::shift_east(bb)),
        ),
    };

    let ep_bb = pos.en_passant.map(Bitboard::from_square).unwrap_or(Bitboard::EMPTY);

    for (captures, offset) in [
        (capt_left(pawns)  & (opp | ep_bb), -1i8),
        (capt_right(pawns) & (opp | ep_bb),  1i8),
    ] {
        for to in captures {
            let from = (to as i8 - dir - offset) as u8;
            let capture = if ep_bb.contains(to) {
                Some(Piece::Pawn)
            } else {
                pos.piece_at(to).map(|(_, p)| p)
            };
            if Bitboard::from_square(to) & promo_rank != Bitboard::EMPTY {
                for promo in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    moves.push(Move { from, to, piece: Piece::Pawn, capture, promotion: Some(promo) });
                }
            } else {
                moves.push(Move { from, to, piece: Piece::Pawn, capture, promotion: None });
            }
        }
    }
}

use super::tables::{rook_attacks, bishop_attacks, queen_attacks};

pub fn generate_sliding_moves(pos: &Position, moves: &mut Vec<Move>) {
    let color = pos.side_to_move;
    let own = pos.color_bb(color);
    let opp = pos.color_bb(color.flip());
    let occ = pos.occupancy();

    for (piece, attack_fn) in [
        (Piece::Rook,   rook_attacks   as fn(u8, Bitboard) -> Bitboard),
        (Piece::Bishop, bishop_attacks as fn(u8, Bitboard) -> Bitboard),
        (Piece::Queen,  queen_attacks  as fn(u8, Bitboard) -> Bitboard),
    ] {
        let mut pieces = pos.piece_bb(color, piece);
        for from in pieces {
            let attacks = attack_fn(from, occ) & !own;
            for to in attacks {
                let capture = if opp.contains(to) { pos.piece_at(to).map(|(_, p)| p) } else { None };
                moves.push(Move { from, to, piece, capture, promotion: None });
            }
        }
    }
}

pub fn generate_king_moves(pos: &Position, moves: &mut Vec<Move>) {
    let color = pos.side_to_move;
    let own = pos.color_bb(color);
    let opp = pos.color_bb(color.flip());
    let occ = pos.occupancy();
    let from = pos.piece_bb(color, Piece::King).lsb();

    let bb = Bitboard::from_square(from);
    let n  = bb.shift_north();
    let s  = bb.shift_south();
    let e  = bb.shift_east();
    let w  = bb.shift_west();
    let attacks = (n | s | e | w | n.shift_east() | n.shift_west() | s.shift_east() | s.shift_west()) & !own;

    for to in attacks {
        let capture = if opp.contains(to) { pos.piece_at(to).map(|(_, p)| p) } else { None };
        moves.push(Move { from, to, piece: Piece::King, capture, promotion: None });
    }

    generate_castling(pos, moves, color, occ);
}

fn generate_castling(pos: &Position, moves: &mut Vec<Move>, color: Color, occ: Bitboard) {
    match color {
        Color::White => {
            if pos.castling.has(CastlingRights::WK) && (occ & Bitboard(0x0000000000000060)) == Bitboard::EMPTY {
                moves.push(Move { from: 4, to: 6, piece: Piece::King, capture: None, promotion: None });
            }
            if pos.castling.has(CastlingRights::WQ) && (occ & Bitboard(0x000000000000000E)) == Bitboard::EMPTY {
                moves.push(Move { from: 4, to: 2, piece: Piece::King, capture: None, promotion: None });
            }
        }
        Color::Black => {
            if pos.castling.has(CastlingRights::BK) && (occ & Bitboard(0x6000000000000000)) == Bitboard::EMPTY {
                moves.push(Move { from: 60, to: 62, piece: Piece::King, capture: None, promotion: None });
            }
            if pos.castling.has(CastlingRights::BQ) && (occ & Bitboard(0x0E00000000000000)) == Bitboard::EMPTY {
                moves.push(Move { from: 60, to: 58, piece: Piece::King, capture: None, promotion: None });
            }
        }
    }
}

pub fn is_in_check(pos: &Position, color: Color) -> bool {
    let king_sq = pos.piece_bb(color, Piece::King).lsb();
    let occ = pos.occupancy();
    let opp = color.flip();

    if knight_attacks()[king_sq as usize] & pos.piece_bb(opp, Piece::Knight) != Bitboard::EMPTY {
        return true;
    }
    if bishop_attacks(king_sq, occ) & (pos.piece_bb(opp, Piece::Bishop) | pos.piece_bb(opp, Piece::Queen)) != Bitboard::EMPTY {
        return true;
    }
    if rook_attacks(king_sq, occ) & (pos.piece_bb(opp, Piece::Rook) | pos.piece_bb(opp, Piece::Queen)) != Bitboard::EMPTY {
        return true;
    }

    let king_bb = Bitboard::from_square(king_sq);
    let pawn_attacks = match color {
        Color::White => king_bb.shift_north().shift_east() | king_bb.shift_north().shift_west(),
        Color::Black => king_bb.shift_south().shift_east() | king_bb.shift_south().shift_west(),
    };
    if pawn_attacks & pos.piece_bb(opp, Piece::Pawn) != Bitboard::EMPTY {
        return true;
    }

    let king_bb = Bitboard::from_square(king_sq);
    let n = king_bb.shift_north();
    let s = king_bb.shift_south();
    let king_zone = n | s | king_bb.shift_east() | king_bb.shift_west()
        | n.shift_east() | n.shift_west() | s.shift_east() | s.shift_west();
    if king_zone & pos.piece_bb(opp, Piece::King) != Bitboard::EMPTY {
        return true;
    }

    false
}
