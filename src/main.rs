use stockstacean::board::Board;

fn main() {
    if let Ok(board) = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1") {
        println!("{}", board);
    }
}
