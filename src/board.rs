use crate::bitboard::{Bitboard, EMPTY};
use crate::color::Color;
use crate::piece::{Piece, PIECES};
use crate::square::Square;

// The board state, represented by multiple bitboards
#[derive(Debug)]
pub struct Board {
    pieces: [Bitboard; 6],
    colors: [Bitboard; 2],
}

impl Board {
    // Set a piece on the board
    pub fn set(&mut self, piece: Piece, color: Color, square: Square) {
        self.pieces[piece as usize].set(square);
        self.colors[color as usize].set(square);
    }

    // Unset a piece on the board
    pub fn unset(&mut self, piece: Piece, color: Color, square: Square) {
        self.pieces[piece as usize].unset(square);
        self.colors[color as usize].unset(square);
    }

    // Get the piece at a specific square on the board
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        let square_bitboard = Bitboard::from(square);

        for piece in PIECES.iter() {
            if (self.pieces[*piece as usize].0 & square_bitboard.0) != EMPTY.0 {
                return Some(*piece);
            }
        }
        None
    }

    // Get the color at a specific square on the board
    pub fn color_on(&self, square: Square) -> Option<Color> {
        let square_bitboard = Bitboard::from(square);

        if self.colors[Color::White as usize].0 & square_bitboard.0 != EMPTY.0 {
            Some(Color::White)
        } else if self.colors[Color::Black as usize].0 & square_bitboard.0 != EMPTY.0 {
            Some(Color::Black)
        } else {
            None
        }
    }
}

// Default is an empty board
impl Default for Board {
    fn default() -> Board {
        Board {
            pieces: [EMPTY; 6],
            colors: [EMPTY; 2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_board() {
        let board: Board = Default::default();

        for square_position in 0..64 {
            let square = Square(square_position);
            assert_eq!(board.piece_on(square), None);
            assert_eq!(board.color_on(square), None);
        }
    }
}
