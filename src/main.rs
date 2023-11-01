extern crate chomp_game;
extern crate prompted;

use prompted::input;
use chomp_game::Board;

fn main() {

    let user_width = input!("Enter a width for the board: ");
    let user_height = input!("Enter a height for the board: ");
    
     // Convert the user input to usize
    let width: usize = user_width.trim().parse().unwrap();
    let height: usize = user_height.trim().parse().unwrap();

    let board = Board::create_board(width,height);
    Board::print_board(&board)
}
