use crate::square::Square;

/// Each bit represents a square on the board
#[derive(Debug)]
pub struct Bitboard(pub u64);
pub const EMPTY: Bitboard = Bitboard(0);

impl Bitboard {
    /// Set a bit on the bitboard
    pub fn set(&mut self, square: Square) {
        self.0 |= Bitboard::from(square).0;
    }

    /// Unset a bit on the bitboard
    pub fn unset(&mut self, square: Square) {
        self.0 ^= Bitboard::from(square).0;
    }
}

impl From<Square> for Bitboard {
    fn from(square: Square) -> Bitboard {
        Bitboard(1 << square.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::File;
    use crate::rank::Rank;

    #[test]
    fn test_bitboard_creation() {
        let a2 = Square::make(File::A, Rank::Second);
        let bitboard = Bitboard::from(a2);
        assert_eq!(bitboard.0, 256);
    }

    #[test]
    fn test_set_bitboard() {
        let a2 = Square::make(File::A, Rank::Second);
        let mut bitboard = EMPTY;
        bitboard.set(a2);
        assert_eq!(bitboard.0, 256);
    }

    #[test]
    fn test_unset_bitboard() {
        let a2 = Square::make(File::A, Rank::Second);
        let mut bitboard = Bitboard::from(a2);
        bitboard.unset(a2);
        assert_eq!(bitboard.0, 0);
    }
}
