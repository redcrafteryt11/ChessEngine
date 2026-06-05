use super::bitboard::Bitboard;

fn sliding_attacks(sq: u8, occ: u64, mask: u64) -> u64 {
    let o = occ & mask;
    let sq_bit = 1u64 << sq;
    let forward = o.wrapping_sub(sq_bit.wrapping_mul(2));
    let reverse = (o.reverse_bits()).wrapping_sub((sq_bit.reverse_bits()).wrapping_mul(2));
    (forward ^ reverse.reverse_bits()) & mask
}

fn rank_mask(sq: u8) -> u64 {
    0xFFu64 << (sq & 56)
}

fn file_mask(sq: u8) -> u64 {
    0x0101010101010101u64 << (sq & 7)
}

fn diag_mask(sq: u8) -> u64 {
    let sq = sq as i32;
    let diag = (sq & 7) - (sq >> 3);
    if diag >= 0 {
        0x8040201008040201u64 >> (diag * 8)
    } else {
        0x8040201008040201u64 << (-diag * 8)
    }
}

fn anti_diag_mask(sq: u8) -> u64 {
    let sq = sq as i32;
    let diag = 7 - (sq & 7) - (sq >> 3);
    if diag >= 0 {
        0x0102040810204080u64 >> (diag * 8)
    } else {
        0x0102040810204080u64 << (-diag * 8)
    }
}

pub fn rook_attacks(sq: u8, occ: Bitboard) -> Bitboard {
    let o = occ.0;
    Bitboard(
        sliding_attacks(sq, o, rank_mask(sq)) |
            sliding_attacks(sq, o, file_mask(sq))
    )
}

pub fn bishop_attacks(sq: u8, occ: Bitboard) -> Bitboard {
    let o = occ.0;
    Bitboard(
        sliding_attacks(sq, o, diag_mask(sq)) |
            sliding_attacks(sq, o, anti_diag_mask(sq))
    )
}

pub fn queen_attacks(sq: u8, occ: Bitboard) -> Bitboard {
    rook_attacks(sq, occ) | bishop_attacks(sq, occ)
}