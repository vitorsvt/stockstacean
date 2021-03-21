use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

impl From<usize> for Piece {
    fn from(value: usize) -> Self {
        match value {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            _ => Piece::King,
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'p' | 'P' => Ok(Piece::Pawn),
            'n' | 'N' => Ok(Piece::Knight),
            'b' | 'B' => Ok(Piece::Bishop),
            'r' | 'R' => Ok(Piece::Rook),
            'q' | 'Q' => Ok(Piece::Queen),
            'k' | 'K' => Ok(Piece::King),
            _ => Err("Invalid char supplied."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_conversion() {
        assert_eq!(Piece::Pawn, Piece::from(0))
    }
}
