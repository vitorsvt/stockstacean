use crate::file::File;
use crate::rank::Rank;

/// A square on the board
#[derive(Copy, Clone, Debug)]
pub struct Square(pub u8);

impl Square {
    /// Create a square from a file and a rank
    pub fn make(file: File, rank: Rank) -> Square {
        Square((rank as u8) << 3 ^ (file as u8))
    }

    /// File of the square
    pub fn file(&self) -> File {
        File::from((self.0 & 7) as usize)
    }

    /// Rank of the square
    pub fn rank(&self) -> Rank {
        Rank::from((self.0 >> 3) as usize)
    }

    pub const A1: Square = Self(0);
    pub const B1: Square = Self(1);
    pub const C1: Square = Self(2);
    pub const D1: Square = Self(3);
    pub const E1: Square = Self(4);
    pub const F1: Square = Self(5);
    pub const G1: Square = Self(6);
    pub const H1: Square = Self(7);

    pub const A2: Square = Self(8);
    pub const B2: Square = Self(9);
    pub const C2: Square = Self(10);
    pub const D2: Square = Self(11);
    pub const E2: Square = Self(12);
    pub const F2: Square = Self(13);
    pub const G2: Square = Self(14);
    pub const H2: Square = Self(15);

    pub const A3: Square = Self(16);
    pub const B3: Square = Self(17);
    pub const C3: Square = Self(18);
    pub const D3: Square = Self(19);
    pub const E3: Square = Self(20);
    pub const F3: Square = Self(21);
    pub const G3: Square = Self(22);
    pub const H3: Square = Self(23);

    pub const A4: Square = Self(24);
    pub const B4: Square = Self(25);
    pub const C4: Square = Self(26);
    pub const D4: Square = Self(27);
    pub const E4: Square = Self(28);
    pub const F4: Square = Self(29);
    pub const G4: Square = Self(30);
    pub const H4: Square = Self(31);

    pub const A5: Square = Self(32);
    pub const B5: Square = Self(33);
    pub const C5: Square = Self(34);
    pub const D5: Square = Self(35);
    pub const E5: Square = Self(36);
    pub const F5: Square = Self(37);
    pub const G5: Square = Self(38);
    pub const H5: Square = Self(39);

    pub const A6: Square = Self(40);
    pub const B6: Square = Self(41);
    pub const C6: Square = Self(42);
    pub const D6: Square = Self(43);
    pub const E6: Square = Self(44);
    pub const F6: Square = Self(45);
    pub const G6: Square = Self(46);
    pub const H6: Square = Self(47);

    pub const A7: Square = Self(48);
    pub const B7: Square = Self(49);
    pub const C7: Square = Self(50);
    pub const D7: Square = Self(51);
    pub const E7: Square = Self(52);
    pub const F7: Square = Self(53);
    pub const G7: Square = Self(54);
    pub const H7: Square = Self(55);

    pub const A8: Square = Self(56);
    pub const B8: Square = Self(57);
    pub const C8: Square = Self(58);
    pub const D8: Square = Self(59);
    pub const E8: Square = Self(60);
    pub const F8: Square = Self(61);
    pub const G8: Square = Self(62);
    pub const H8: Square = Self(63);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_from_pos() {
        let a2 = Square::make(File::A, Rank::Second);
        assert_eq!(a2.0, 8);
    }
    #[test]
    fn test_square_file_and_rank() {
        for rank in 0..8 {
            for file in 0..8 {
                let square = Square::make(File::from(file), Rank::from(rank));
                assert_eq!(square.file() as usize, file);
                assert_eq!(square.rank() as usize, rank);
            }
        }
    }
}
