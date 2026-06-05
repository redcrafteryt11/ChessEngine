use std::ops::{BitAnd, BitOr, BitXor, Not, Shl, Shr};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Self = Self(0);
    pub const FULL:  Self = Self(u64::MAX);

    pub const FILE_A: Self = Self(0x0101010101010101);
    pub const FILE_H: Self = Self(0x8080808080808080);
    pub const FILE_B: Self = Self(0x0202020202020202);
    pub const FILE_G: Self = Self(0x4040404040404040);
    pub const RANK_1: Self = Self(0x00000000000000FF);
    pub const RANK_2: Self = Self(0x000000000000FF00);
    pub const RANK_3: Self = Self(0x0000000000FF0000);
    pub const RANK_4: Self = Self(0x00000000FF000000);
    pub const RANK_5: Self = Self(0x000000FF00000000);
    pub const RANK_6: Self = Self(0x0000FF0000000000);
    pub const RANK_7: Self = Self(0x00FF000000000000);
    pub const RANK_8: Self = Self(0xFF00000000000000);


    #[inline] pub fn from_square(sq: u8) -> Self { Self(1u64 << sq) }
    #[inline] pub fn is_empty(self) -> bool { self.0 == 0 }
    #[inline] pub fn popcount(self) -> u32 { self.0.count_ones() }
    #[inline] pub fn lsb(self) -> u8 { self.0.trailing_zeros() as u8 }
    #[inline] pub fn pop_lsb(&mut self) -> u8 {
        let sq = self.lsb();
        self.0 &= self.0 - 1;
        sq
    }
    #[inline] pub fn contains(self, sq: u8) -> bool {
        (self.0 >> sq) & 1 == 1
    }
    #[inline] pub fn shift_north(self) -> Self { Self(self.0 << 8) }
    #[inline] pub fn shift_south(self) -> Self { Self(self.0 >> 8) }
    #[inline] pub fn shift_east(self)  -> Self { Self((self.0 & !Self::FILE_H.0) << 1) }
    #[inline] pub fn shift_west(self)  -> Self { Self((self.0 & !Self::FILE_A.0) >> 1) }
}

impl Iterator for Bitboard {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.is_empty() { None } else { Some(self.pop_lsb()) }
    }
}

impl BitAnd for Bitboard { type Output = Self; fn bitand(self, r: Self) -> Self { Self(self.0 & r.0) } }
impl BitOr  for Bitboard { type Output = Self; fn bitor (self, r: Self) -> Self { Self(self.0 | r.0) } }
impl BitXor for Bitboard { type Output = Self; fn bitxor(self, r: Self) -> Self { Self(self.0 ^ r.0) } }
impl Not    for Bitboard { type Output = Self; fn not(self)             -> Self { Self(!self.0) } }
impl Shl<u8> for Bitboard { type Output = Self; fn shl(self, r: u8) -> Self { Self(self.0 << r) } }
impl Shr<u8> for Bitboard { type Output = Self; fn shr(self, r: u8) -> Self { Self(self.0 >> r) } }

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = rank * 8 + file;
                write!(f, "{} ", if self.contains(sq) { '1' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}