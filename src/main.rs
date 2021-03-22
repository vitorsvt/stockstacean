use stockstacean::board::Board;
use stockstacean::square::Square;

fn main() {
    let mut board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    board.make_move(Square::A2, Square::A3);
    println!("{}", board);
}
