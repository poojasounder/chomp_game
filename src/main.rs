extern crate chomp_game;
extern crate prompted;

use chomp_game::Board;
use prompted::input;

fn main() {
    println!("Welcome to my chomp game");
    println!("Let's create your custom board!");
    let user_width = input!("Enter a width for the board: ");
    let user_height = input!("Enter a height for the board: ");

    // Convert the user input to usize
    let width: usize = user_width.trim().parse().unwrap();
    let height: usize = user_height.trim().parse().unwrap();

    let mut board = Board::create_board(width, height);
    Board::print_board(&board);
    while Board::is_game_over(&board) == false {
        let user_move_row: usize = input!("Enter a row for the square you want to remove: ")
            .trim()
            .parse()
            .unwrap();
        let user_move_col: usize = input!("Enter a col for the square you want to remove: ")
            .trim()
            .parse()
            .unwrap();

        Board::chomp(&mut board, user_move_row - 1, user_move_col - 1);
        Board::print_board(&board);
        if let Some(winning_move) = Board::winning_move(&board) {
            println!(
                "The winning move is: ({}, {})",
                winning_move.0, winning_move.1
            );
            Board::chomp(&mut board, winning_move.0, winning_move.1);
        };
        Board::print_board(&board);
    }
}
