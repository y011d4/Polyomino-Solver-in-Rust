mod env;
mod solver;
use env::Board;
use env::Piece;
use solver::solve;

fn main() {
    let nr: usize = 10;
    let nc: usize = 6;
    let mut board = Board::new(nr, nc);

    let mut pieces: Vec<Piece> = Vec::new();

    /*
    pieces.push(Piece::new("010111", 2, 3, true, true, board.nc));
    pieces.push(Piece::new("110111", 3, 2, true, true, board.nc));
    pieces.push(Piece::new("111",    1, 3, true, true, board.nc));
    */

    /*
    pieces.push(Piece::new("1111", 2, 2, false, false, nr, nc));
    pieces.push(Piece::new("1111", 2, 2, false, false, nr, nc));
    pieces.push(Piece::new("1111", 2, 2, false, false, nr, nc));
    pieces.push(Piece::new("1111", 2, 2, false, false, nr, nc));
    */
    /*
    pieces.push(Piece::new("010111010", 3, 3, 1, 1, board.nc));
    pieces.push(Piece::new("111101",    2, 3, 4, 1, board.nc));
    pieces.push(Piece::new("110011001", 3, 3, 4, 1, board.nc));
    pieces.push(Piece::new("110011010", 3, 3, 2, 1, board.nc));
    pieces.push(Piece::new("110010011", 3, 3, 2, 2, board.nc));
    pieces.push(Piece::new("111110",    2, 3, 4, 2, board.nc));
    pieces.push(Piece::new("11100011",  2, 4, 4, 2, board.nc));
    pieces.push(Piece::new("11110100",  2, 4, 4, 2, board.nc));
    pieces.push(Piece::new("111010010", 3, 3, 4, 1, board.nc));
    pieces.push(Piece::new("11111000",  2, 4, 4, 2, board.nc));
    pieces.push(Piece::new("111100100", 3, 3, 4, 1, board.nc));
    pieces.push(Piece::new("11111",     1, 5, 2, 1, board.nc));
    */
    pieces.push(Piece::new("010111010", 3, 3, true, true, board.nc));
    pieces.push(Piece::new("111101",    2, 3, true, true, board.nc));
    pieces.push(Piece::new("110011001", 3, 3, true, true, board.nc));
    pieces.push(Piece::new("110011010", 3, 3, false, true, board.nc));
    pieces.push(Piece::new("110010011", 3, 3, true, true, board.nc));
    pieces.push(Piece::new("111110",    2, 3, true, true, board.nc));
    pieces.push(Piece::new("11100011",  2, 4, true, true, board.nc));
    pieces.push(Piece::new("11110100",  2, 4, true, true, board.nc));
    pieces.push(Piece::new("111010010", 3, 3, true, true, board.nc));
    pieces.push(Piece::new("11111000",  2, 4, true, true, board.nc));
    pieces.push(Piece::new("111100100", 3, 3, true, true, board.nc));
    pieces.push(Piece::new("11111",     1, 5, true, true, board.nc));

    let mut cnt = 0;
    solve(&mut board, &mut pieces, 0, &mut cnt);
}
