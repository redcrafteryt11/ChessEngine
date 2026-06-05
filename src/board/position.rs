use super::{bitboard::Bitboard, piece::{Color, Piece}};

#[derive(Debug, Clone)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    pub const NONE:  Self = Self(0b0000);
    pub const ALL:   Self = Self(0b1111);
    pub const WK: u8 = 0b0001;
    pub const WQ: u8 = 0b0010;
    pub const BK: u8 = 0b0100;
    pub const BQ: u8 = 0b1000;

    pub fn has(&self, flag: u8) -> bool { self.0 & flag != 0 }
    pub fn remove(&mut self, flag: u8) { self.0 &= !flag; }
}

#[derive(Debug, Clone)]
pub struct Position {
    pieces: [[Bitboard; 6]; 2],
    pub side_to_move: Color,
    pub castling: CastlingRights,
    pub en_passant: Option<u8>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
}

impl Position {
    pub fn empty() -> Self {
        Self {
            pieces: [[Bitboard::EMPTY; 6]; 2],
            side_to_move: Color::White,
            castling: CastlingRights::NONE,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn piece_bb(&self, color: Color, piece: Piece) -> Bitboard {
        self.pieces[color as usize][piece as usize]
    }

    pub fn piece_bb_mut(&mut self, color: Color, piece: Piece) -> &mut Bitboard {
        &mut self.pieces[color as usize][piece as usize]
    }

    pub fn occupancy(&self) -> Bitboard {
        self.color_bb(Color::White) | self.color_bb(Color::Black)
    }

    pub fn color_bb(&self, color: Color) -> Bitboard {
        self.pieces[color as usize].iter().copied().fold(Bitboard::EMPTY, |a, b| a | b)
    }

    pub fn place_piece(&mut self, color: Color, piece: Piece, sq: u8) {
        *self.piece_bb_mut(color, piece) = self.piece_bb(color, piece) | Bitboard::from_square(sq);
    }

    pub fn remove_piece(&mut self, color: Color, piece: Piece, sq: u8) {
        *self.piece_bb_mut(color, piece) = self.piece_bb(color, piece) & !Bitboard::from_square(sq);
    }

    pub fn piece_at(&self, sq: u8) -> Option<(Color, Piece)> {
        for color in [Color::White, Color::Black] {
            for piece in [Piece::Pawn, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King] {
                if self.piece_bb(color, piece).contains(sq) {
                    return Some((color, piece));
                }
            }
        }
        None
    }
}


use std::str::FromStr;

#[derive(Debug)]
pub struct FenError(pub String);

impl Position {
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err(FenError("too few fields".into()));
        }

        let mut pos = Position::empty();
        let mut sq: u8 = 56;

        for ch in parts[0].chars() {
            match ch {
                '/' => sq = sq.saturating_sub(16),
                '1'..='8' => sq += ch as u8 - b'0',
                _ => {
                    let (color, piece) = Self::char_to_piece(ch)
                        .ok_or_else(|| FenError(format!("unknown piece: {ch}")))?;
                    pos.place_piece(color, piece, sq);
                    sq += 1;
                }
            }
        }

        pos.side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenError("invalid side".into())),
        };

        pos.castling = CastlingRights(0);
        for ch in parts[2].chars() {
            match ch {
                'K' => pos.castling.0 |= CastlingRights::WK,
                'Q' => pos.castling.0 |= CastlingRights::WQ,
                'k' => pos.castling.0 |= CastlingRights::BK,
                'q' => pos.castling.0 |= CastlingRights::BQ,
                '-' => {}
                _ => return Err(FenError(format!("invalid castling: {ch}"))),
            }
        }

        pos.en_passant = match parts[3] {
            "-" => None,
            s => Some(Self::square_from_str(s)?),
        };

        if parts.len() >= 5 {
            pos.halfmove_clock = parts[4].parse().unwrap_or(0);
        }
        if parts.len() >= 6 {
            pos.fullmove_number = parts[5].parse().unwrap_or(1);
        }

        Ok(pos)
    }

    fn char_to_piece(ch: char) -> Option<(Color, Piece)> {
        let color = if ch.is_uppercase() { Color::White } else { Color::Black };
        let piece = match ch.to_ascii_lowercase() {
            'p' => Piece::Pawn,
            'n' => Piece::Knight,
            'b' => Piece::Bishop,
            'r' => Piece::Rook,
            'q' => Piece::Queen,
            'k' => Piece::King,
            _ => return None,
        };
        Some((color, piece))
    }

    fn square_from_str(s: &str) -> Result<u8, FenError> {
        let b = s.as_bytes();
        if b.len() < 2 {
            return Err(FenError("invalid square".into()));
        }
        let file = b[0].checked_sub(b'a').ok_or_else(|| FenError("invalid file".into()))?;
        let rank = b[1].checked_sub(b'1').ok_or_else(|| FenError("invalid rank".into()))?;
        if file > 7 || rank > 7 {
            return Err(FenError("square out of range".into()));
        }
        Ok(rank * 8 + file)
    }
}