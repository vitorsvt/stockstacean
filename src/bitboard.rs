#[derive(Debug)]
pub struct Bitboard(pub u64);

pub const EMPTY: Bitboard = Bitboard(0);

impl Bitboard {
    pub fn set(&mut self, rank: u8, file: u8) {
        if rank < 8 && file < 8 {
            self.0 |= 1u64 << (rank * 8 + file);
        }
    }

    pub fn unset(&mut self, rank: u8, file: u8) {
        if rank < 8 && file < 8 {
            self.0 ^= 1u64 << (rank * 8 + file);
        }
    }
}

#[test]
fn test_set_bitboard() {
    let mut a2_bitboard = EMPTY;
    a2_bitboard.set(1, 0);
    assert_eq!(a2_bitboard.0, 256);
}

#[test]
fn test_unset_bitboard() {
    let mut a2_bitboard = Bitboard(256);
    a2_bitboard.unset(1, 0);
    assert_eq!(a2_bitboard.0, 0);
}
