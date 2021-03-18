use crate::file::File;
use crate::rank::Rank;

// A square on the board
pub struct Square(pub u8);

impl Square {
    // Create a square from a file and a rank
    pub fn from_pos(file: File, rank: Rank) -> Square {
        Square((rank as u8) << 3 ^ (file as u8))
    }

    // File of the square
    pub fn file(&self) -> File {
        File::from((self.0 & 7) as usize)
    }

    // Rank of the square
    pub fn rank(&self) -> Rank {
        Rank::from((self.0 >> 3) as usize)
    }
}

#[test]
fn test_square_from_pos() {
    let a2 = Square::from_pos(File::A, Rank::Second);

    assert_eq!(a2.0, 8);
}

#[test]
fn test_square_file_and_rank() {
    for rank in 0..8 {
        for file in 0..8 {
            let square = Square::from_pos(File::from(file), Rank::from(rank));

            assert_eq!(square.file() as usize, file);
            assert_eq!(square.rank() as usize, rank);
        }
    }
}
