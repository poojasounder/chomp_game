extern crate chomp_game;

use chomp_game::Board;
fn main() {
    let board = Board::create_board(4,5);
    Board::print_board(&board)
}
