use crate::bitboard::{Bitboard, EMPTY};
use crate::color::Color;
use crate::file::File;
use crate::piece::{Piece, PIECES};
use crate::rank::Rank;
use crate::square::Square;
use std::convert::TryInto;
use std::fmt;

/// The board state, represented by multiple bitboards
#[derive(Debug)]
pub struct Board {
    pieces: [Bitboard; 6],
    colors: [Bitboard; 2],
    turn: Color,
}

impl Board {
    /// Set a piece on the board
    pub fn set(&mut self, piece: Piece, color: Color, square: Square) {
        self.pieces[piece as usize].set(square);
        self.colors[color as usize].set(square);
    }

    /// Unset a piece on the board
    pub fn unset(&mut self, piece: Piece, color: Color, square: Square) {
        self.pieces[piece as usize].unset(square);
        self.colors[color as usize].unset(square);
    }

    /// Get the piece at a specific square on the board
    pub fn piece_on(&self, square: Square) -> Option<Piece> {
        let square_bitboard = Bitboard::from(square);

        for piece in PIECES.iter() {
            if (self.pieces[*piece as usize].0 & square_bitboard.0) != EMPTY.0 {
                return Some(*piece);
            }
        }
        None
    }

    /// Get the color at a specific square on the board
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

    /// XOR operation on all bitboards
    fn xor(&mut self, piece: Piece, color: Color, bitboard: Bitboard) {
        self.pieces[piece as usize].0 ^= bitboard.0;
        self.colors[color as usize].0 ^= bitboard.0;
    }

    /// Move a piece on the board
    pub fn make_move(&mut self, source: Square, destination: Square) {
        let source_bitboard = Bitboard::from(source);
        let destination_bitboard = Bitboard::from(destination);

        let destination_piece = self.piece_on(destination);

        if let Some(piece_moved) = self.piece_on(source) {
            self.xor(piece_moved, self.turn, source_bitboard);
            self.xor(piece_moved, self.turn, destination_bitboard);
            if let Some(piece_captured) = destination_piece {
                self.xor(piece_captured, !self.turn, destination_bitboard);
            }
        }

        self.turn = !self.turn;
    }

    /// Creates a board from a FEN string
    pub fn from_fen(fen: &str) -> Result<Board, &'static str> {
        let splitted: Vec<&str> = fen.split(' ').collect();
        let symbols = splitted[0];

        let mut rank = Rank::Eighth;
        let mut file = File::A;
        let mut board: Board = Default::default();

        for symbol in symbols.chars() {
            if symbol == '/' {
                rank = rank.down();
                file = File::A;
            } else if ('1'..'9').contains(&symbol) {
                file = File::from(file as usize + symbol as usize);
            } else if let Ok(piece) = symbol.try_into() {
                board.set(piece, symbol.into(), Square::make(file, rank));
                file = file.right();
            } else {
                return Err("Invalid FEN string supplied.");
            }
        }
        Ok(board)
    }
}

// Default is an empty board
impl Default for Board {
    fn default() -> Board {
        Board {
            pieces: [EMPTY; 6],
            colors: [EMPTY; 2],
            turn: Color::White,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string: String = "".to_owned();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Square::make(file.into(), rank.into());

                if let Some(piece) = self.piece_on(square) {
                    string.push(if self.color_on(square) == Some(Color::Black) {
                        char::from(piece)
                    } else {
                        // to_uppercase returns an iterator
                        // next / unwrap may not be the best practice
                        char::from(piece).to_uppercase().next().unwrap()
                    });
                } else {
                    string.push('.');
                }
            }
            if rank != 0 {
                string.push('\n');
            }
        }

        write!(f, "{}", string)
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

    #[test]
    fn test_board_from_fen() {
        let board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        for square_position in 0..64 {
            let square = Square(square_position);
            // Currently checking piece existence and color
            if (0..=15).contains(&square_position) {
                assert_ne!(board.piece_on(square), None);
                assert_eq!(board.color_on(square), Some(Color::White));
            } else if (48..=63).contains(&square_position) {
                assert_ne!(board.piece_on(square), None);
                assert_eq!(board.color_on(square), Some(Color::Black));
            } else {
                assert_eq!(board.piece_on(square), None);
                assert_eq!(board.color_on(square), None);
            }
        }
    }
}
