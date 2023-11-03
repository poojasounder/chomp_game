extern crate chomp_game;
extern crate prompted;

use prompted::input;
use chomp_game::Board;


fn main(){

    let user_width = input!("Enter a width for the board: ");
    let user_height = input!("Enter a height for the board: ");

     // Convert the user input to usize
    let width: usize = user_width.trim().parse().unwrap();
    let height: usize = user_height.trim().parse().unwrap();

    let mut board = Board::create_board(width,height);
    Board::print_board(&board);
    let user_move_row: usize = input!("Enter a row for the square you want to remove: ").trim().parse().unwrap();
    let user_move_col:usize = input!("Enter a col for the square you want to remove: ").trim().parse().unwrap();
    
    Board::chomp(&mut board,user_move_row-1,user_move_col-1);
    Board::print_board(&board);
    Board::winning_move(&board);
}
