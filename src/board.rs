use crate::bitboard::{Bitboard, EMPTY};
use crate::color::Color;
use crate::file::File;
use crate::piece::{Piece, PIECES};
use crate::rank::Rank;
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

    // Creates a board from a FEN string
    pub fn from_fen(fen: &str) -> Result<Board, &'static str> {
        let splitted: Vec<&str> = fen.split(' ').collect();
        let symbols = splitted[0];

        let mut rank = Rank::Eighth;
        let mut file = File::A;
        let mut board: Board = Default::default();

        for symbol in symbols.chars() {
            match symbol {
                '/' => {
                    rank = rank.down();
                    file = File::A;
                }
                '1'..='8' => {
                    file = File::from(file as usize + symbol as usize);
                }
                'p' => {
                    board.set(Piece::Pawn, Color::Black, Square::make(file, rank));
                    file = file.right();
                }
                'P' => {
                    board.set(Piece::Pawn, Color::White, Square::make(file, rank));
                    file = file.right();
                }
                'n' => {
                    board.set(Piece::Knight, Color::Black, Square::make(file, rank));
                    file = file.right();
                }
                'N' => {
                    board.set(Piece::Knight, Color::White, Square::make(file, rank));
                    file = file.right();
                }
                'b' => {
                    board.set(Piece::Bishop, Color::Black, Square::make(file, rank));
                    file = file.right();
                }
                'B' => {
                    board.set(Piece::Bishop, Color::White, Square::make(file, rank));
                    file = file.right();
                }
                'r' => {
                    board.set(Piece::Rook, Color::Black, Square::make(file, rank));
                    file = file.right();
                }
                'R' => {
                    board.set(Piece::Rook, Color::White, Square::make(file, rank));
                    file = file.right();
                }
                'q' => {
                    board.set(Piece::Queen, Color::Black, Square::make(file, rank));
                    file = file.right();
                }
                'Q' => {
                    board.set(Piece::Queen, Color::White, Square::make(file, rank));
                    file = file.right();
                }
                'k' => {
                    board.set(Piece::King, Color::Black, Square::make(file, rank));
                    file = file.right();
                }
                'K' => {
                    board.set(Piece::King, Color::White, Square::make(file, rank));
                    file = file.right();
                }
                _ => return Err("Invalid FEN string supplied."),
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
