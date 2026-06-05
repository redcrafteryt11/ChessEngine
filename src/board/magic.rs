use super::bitboard::Bitboard;

pub struct Magic {
    pub mask:   u64,
    pub number: u64,
    pub shift:  u8,
}

impl Magic {
    #[inline]
    pub fn index(&self, occ: u64) -> usize {
        ((occ & self.mask).wrapping_mul(self.number) >> self.shift) as usize
    }
}

pub static ROOK_MAGICS: [Magic; 64] = [
    Magic { mask: 0x000101010101017E, number: 0x0080001020400080, shift: 52 },
    Magic { mask: 0x000202020202027C, number: 0x0040001000200040, shift: 53 },
    Magic { mask: 0x000404040404047A, number: 0x0080081000200080, shift: 53 },
    Magic { mask: 0x0008080808080876, number: 0x0080040800100080, shift: 53 },
    Magic { mask: 0x001010101010106E, number: 0x0080020400080080, shift: 53 },
    Magic { mask: 0x002020202020205E, number: 0x0080010200040080, shift: 53 },
    Magic { mask: 0x004040404040403E, number: 0x0080008001000200, shift: 53 },
    Magic { mask: 0x008080808080807E, number: 0x0080002040800100, shift: 52 },
    Magic { mask: 0x0001010101017E00, number: 0x0000800020400080, shift: 53 },
    Magic { mask: 0x0002020202027C00, number: 0x0000400020005000, shift: 54 },
    Magic { mask: 0x0004040404047A00, number: 0x0000801000200080, shift: 54 },
    Magic { mask: 0x0008080808087600, number: 0x0000800800100080, shift: 54 },
    Magic { mask: 0x0010101010106E00, number: 0x0000800400080080, shift: 54 },
    Magic { mask: 0x0020202020205E00, number: 0x0000800200040080, shift: 54 },
    Magic { mask: 0x0040404040403E00, number: 0x0000800100020080, shift: 54 },
    Magic { mask: 0x0080808080807E00, number: 0x0000800040800100, shift: 53 },
    Magic { mask: 0x00010101017E0100, number: 0x0000208000400080, shift: 53 },
    Magic { mask: 0x00020202027C0200, number: 0x0000404000201000, shift: 54 },
    Magic { mask: 0x00040404047A0400, number: 0x0000808010002000, shift: 54 },
    Magic { mask: 0x0008080808760800, number: 0x0000808008001000, shift: 54 },
    Magic { mask: 0x00101010106E1000, number: 0x0000808004000800, shift: 54 },
    Magic { mask: 0x00202020205E2000, number: 0x0000808002000400, shift: 54 },
    Magic { mask: 0x00404040403E4000, number: 0x0000010100020004, shift: 54 },
    Magic { mask: 0x00808080807E8000, number: 0x0000020000408104, shift: 53 },
    Magic { mask: 0x000101017E010100, number: 0x0000208080004000, shift: 53 },
    Magic { mask: 0x000202027C020200, number: 0x0000200040005000, shift: 54 },
    Magic { mask: 0x000404047A040400, number: 0x0000100080200080, shift: 54 },
    Magic { mask: 0x0008080876080800, number: 0x0000080080100080, shift: 54 },
    Magic { mask: 0x001010106E101000, number: 0x0000040080080080, shift: 54 },
    Magic { mask: 0x002020205E202000, number: 0x0000020080040080, shift: 54 },
    Magic { mask: 0x004040403E404000, number: 0x0000010080800200, shift: 54 },
    Magic { mask: 0x008080807E808000, number: 0x0000800080004100, shift: 53 },
    Magic { mask: 0x0001017E01010100, number: 0x0000204000800080, shift: 53 },
    Magic { mask: 0x0002027C02020200, number: 0x0000200040401000, shift: 54 },
    Magic { mask: 0x0004047A04040400, number: 0x0000100080802000, shift: 54 },
    Magic { mask: 0x0008087608080800, number: 0x0000080080801000, shift: 54 },
    Magic { mask: 0x0010106E10101000, number: 0x0000040080800800, shift: 54 },
    Magic { mask: 0x0020205E20202000, number: 0x0000020080800400, shift: 54 },
    Magic { mask: 0x0040403E40404000, number: 0x0000020001010004, shift: 54 },
    Magic { mask: 0x0080807E80808000, number: 0x0000800040800100, shift: 53 },
    Magic { mask: 0x00017E0101010100, number: 0x0000204000808000, shift: 53 },
    Magic { mask: 0x00027C0202020200, number: 0x0000200040008080, shift: 54 },
    Magic { mask: 0x00047A0404040400, number: 0x0000100020008080, shift: 54 },
    Magic { mask: 0x0008760808080800, number: 0x0000080010008080, shift: 54 },
    Magic { mask: 0x00106E1010101000, number: 0x0000040008008080, shift: 54 },
    Magic { mask: 0x00205E2020202000, number: 0x0000020004008080, shift: 54 },
    Magic { mask: 0x00403E4040404000, number: 0x0000010002008080, shift: 54 },
    Magic { mask: 0x00807E8080808000, number: 0x0000004081020004, shift: 53 },
    Magic { mask: 0x007E010101010100, number: 0x0000204000800080, shift: 52 },
    Magic { mask: 0x007C020202020200, number: 0x0000200040008080, shift: 53 },
    Magic { mask: 0x007A040404040400, number: 0x0000100020008080, shift: 53 },
    Magic { mask: 0x0076080808080800, number: 0x0000080010008080, shift: 53 },
    Magic { mask: 0x006E101010101000, number: 0x0000040008008080, shift: 53 },
    Magic { mask: 0x005E202020202000, number: 0x0000020004008080, shift: 53 },
    Magic { mask: 0x003E404040404000, number: 0x0000800100020080, shift: 53 },
    Magic { mask: 0x007E808080808000, number: 0x0000800041000080, shift: 52 },
    Magic { mask: 0x7E01010101010100, number: 0x00FFFCDDFCED714A, shift: 52 },
    Magic { mask: 0x7C02020202020200, number: 0x007FFCDDFCED714A, shift: 53 },
    Magic { mask: 0x7A04040404040400, number: 0x003FFFCDFFD88096, shift: 53 },
    Magic { mask: 0x7608080808080800, number: 0x0003FFD7EFFFDE96, shift: 53 },
    Magic { mask: 0x6E10101010101000, number: 0x0001FFFC0A7FA8C4, shift: 53 },
    Magic { mask: 0x5E20202020202000, number: 0x0000FFFC18280028, shift: 53 },
    Magic { mask: 0x3E40404040404000, number: 0x00007FFF7FBFD008, shift: 53 },
    Magic { mask: 0x7E80808080808000, number: 0x0000001FFFE4FFFA, shift: 52 },
];

pub static BISHOP_MAGICS: [Magic; 64] = [
    Magic { mask: 0x0040201008040200, number: 0x0010020400810100, shift: 58 },
    Magic { mask: 0x0000402010080400, number: 0x0000804010040200, shift: 59 },
    Magic { mask: 0x0000004020100A00, number: 0x0000020080100080, shift: 59 },
    Magic { mask: 0x0000000040221400, number: 0x0000010040080040, shift: 59 },
    Magic { mask: 0x0000000002442800, number: 0x0000008020040020, shift: 59 },
    Magic { mask: 0x0000000204085000, number: 0x0000004010020010, shift: 59 },
    Magic { mask: 0x0000020408102000, number: 0x0000002008010008, shift: 59 },
    Magic { mask: 0x0002040810204000, number: 0x0000001004020004, shift: 58 },
    Magic { mask: 0x0020100804020000, number: 0x0000804010040200, shift: 59 },
    Magic { mask: 0x0040201008040000, number: 0x0000402008020100, shift: 59 },
    Magic { mask: 0x00004020100A0000, number: 0x0000020080100080, shift: 59 },
    Magic { mask: 0x0000004022140000, number: 0x0000010040080040, shift: 59 },
    Magic { mask: 0x0000000244280000, number: 0x0000008020040020, shift: 59 },
    Magic { mask: 0x0000020408500000, number: 0x0000004010020010, shift: 59 },
    Magic { mask: 0x0002040810200000, number: 0x0000804010040200, shift: 59 },
    Magic { mask: 0x0004081020400000, number: 0x0000402008020100, shift: 59 },
    Magic { mask: 0x0010080402000200, number: 0x0000201004010080, shift: 59 },
    Magic { mask: 0x0020100804000400, number: 0x0000100802008040, shift: 59 },
    Magic { mask: 0x004020100A000A00, number: 0x0000080401004020, shift: 57 },
    Magic { mask: 0x0000402214001400, number: 0x0000040200802010, shift: 57 },
    Magic { mask: 0x0000024428002800, number: 0x0000020100401008, shift: 57 },
    Magic { mask: 0x0002040850005000, number: 0x0000010080200804, shift: 57 },
    Magic { mask: 0x0004081020002000, number: 0x0000804010040200, shift: 59 },
    Magic { mask: 0x0008102040004000, number: 0x0000402008020100, shift: 59 },
    Magic { mask: 0x0008040200020400, number: 0x0000201004010080, shift: 59 },
    Magic { mask: 0x0010080400040800, number: 0x0000100802008040, shift: 59 },
    Magic { mask: 0x0020100A000A1000, number: 0x0000080401004020, shift: 57 },
    Magic { mask: 0x0040221400142200, number: 0x0000040200802010, shift: 55 },
    Magic { mask: 0x0002442800284400, number: 0x0000020100401008, shift: 55 },
    Magic { mask: 0x0004085000500800, number: 0x0000010080200804, shift: 57 },
    Magic { mask: 0x0008102000201000, number: 0x0000804010040200, shift: 59 },
    Magic { mask: 0x0010204000402000, number: 0x0000402008020100, shift: 59 },
    Magic { mask: 0x0004020002040800, number: 0x0000201004010080, shift: 59 },
    Magic { mask: 0x0008040004081000, number: 0x0000100802008040, shift: 59 },
    Magic { mask: 0x00100A000A102000, number: 0x0000080401004020, shift: 57 },
    Magic { mask: 0x0022140014224000, number: 0x0000040200802010, shift: 55 },
    Magic { mask: 0x0044280028440200, number: 0x0000020100401008, shift: 55 },
    Magic { mask: 0x0008500050080400, number: 0x0000010080200804, shift: 57 },
    Magic { mask: 0x0010200020100800, number: 0x0000804010040200, shift: 59 },
    Magic { mask: 0x0020400040201000, number: 0x0000402008020100, shift: 59 },
    Magic { mask: 0x0002000204081000, number: 0x0000201004010080, shift: 59 },
    Magic { mask: 0x0004000408102000, number: 0x0000100802008040, shift: 59 },
    Magic { mask: 0x000A000A10204000, number: 0x0000080401004020, shift: 57 },
    Magic { mask: 0x0014001422400000, number: 0x0000040200802010, shift: 57 },
    Magic { mask: 0x0028002844020000, number: 0x0000020100401008, shift: 57 },
    Magic { mask: 0x0050005008040200, number: 0x0000010080200804, shift: 57 },
    Magic { mask: 0x0020002010080400, number: 0x0000804010040200, shift: 59 },
    Magic { mask: 0x0040004020100800, number: 0x0000402008020100, shift: 59 },
    Magic { mask: 0x0000020408102000, number: 0x0000201004010080, shift: 59 },
    Magic { mask: 0x0000040810204000, number: 0x0000100802008040, shift: 59 },
    Magic { mask: 0x00000A1020400000, number: 0x0000080401004020, shift: 59 },
    Magic { mask: 0x0000142240000000, number: 0x0000040200802010, shift: 59 },
    Magic { mask: 0x0000284402000000, number: 0x0000020100401008, shift: 59 },
    Magic { mask: 0x0000500804020000, number: 0x0000010080200804, shift: 59 },
    Magic { mask: 0x0000201008040200, number: 0x0000804010040200, shift: 59 },
    Magic { mask: 0x0000402010080400, number: 0x0000402008020100, shift: 59 },
    Magic { mask: 0x0002040810204000, number: 0x0020401008040200, shift: 58 },
    Magic { mask: 0x0004081020400000, number: 0x0010200804020100, shift: 59 },
    Magic { mask: 0x000A102040000000, number: 0x0008100402010080, shift: 59 },
    Magic { mask: 0x0014224000000000, number: 0x0004080201008040, shift: 59 },
    Magic { mask: 0x0028440200000000, number: 0x0002040100804020, shift: 59 },
    Magic { mask: 0x0050080402000000, number: 0x0001020080402010, shift: 59 },
    Magic { mask: 0x0020100804020000, number: 0x0000810040201008, shift: 59 },
    Magic { mask: 0x0040201008040200, number: 0x0010020400810100, shift: 58 },
];

pub fn rook_attacks_slow(sq: u8, occ: u64) -> u64 {
    let mut attacks = 0u64;
    let r = sq / 8;
    let f = sq % 8;
    for i in (f + 1)..8 { attacks |= 1 << (r * 8 + i); if occ & (1 << (r * 8 + i)) != 0 { break; } }
    for i in (0..f).rev() { attacks |= 1 << (r * 8 + i); if occ & (1 << (r * 8 + i)) != 0 { break; } }
    for i in (r + 1)..8 { attacks |= 1 << (i * 8 + f); if occ & (1 << (i * 8 + f)) != 0 { break; } }
    for i in (0..r).rev() { attacks |= 1 << (i * 8 + f); if occ & (1 << (i * 8 + f)) != 0 { break; } }
    attacks
}

pub fn bishop_attacks_slow(sq: u8, occ: u64) -> u64 {
    let mut attacks = 0u64;
    let r = sq / 8;
    let f = sq % 8;
    let dirs: [(i8, i8); 4] = [(1,1),(1,-1),(-1,1),(-1,-1)];
    for (dr, df) in dirs {
        let (mut cr, mut cf) = (r as i8 + dr, f as i8 + df);
        while cr >= 0 && cr < 8 && cf >= 0 && cf < 8 {
            let bit = 1u64 << (cr * 8 + cf);
            attacks |= bit;
            if occ & bit != 0 { break; }
            cr += dr; cf += df;
        }
    }
    attacks
}