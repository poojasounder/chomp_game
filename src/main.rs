extern crate chomp_game;
extern crate prompted;

use chomp_game::MAX_HEIGHT;
use chomp_game::MAX_WIDTH;
use chomp_game::Board;
use prompted::input;



fn main() {
    println!("Welcome to my chomp game");
    println!("Let's create your custom board!");

    // read user input for their desired board size
    let mut user_width = input!("Enter a width for the board: ").trim().parse().unwrap();
    let mut user_height = input!("Enter a height for the board: ").trim().parse().unwrap();

    while user_width > MAX_WIDTH || user_height > MAX_HEIGHT{
        println!();
        println!("Invalid width or height!!!");
        println!("Width must be lesser than or equal to {} AND Height must be lesser than or equal to {}", MAX_WIDTH,MAX_HEIGHT);

        user_width = input!("Enter a width for the board: ").trim().parse().unwrap();
        user_height = input!("Enter a height for the board: ").trim().parse().unwrap();
    }

    // Create the board of the desired width and height
    let mut board = Board::create_board(user_width, user_height);

    // display the board
    Board::print_board(&board);

    // Run this loop till game is over
    'outer: while Board::is_game_over(&board) == false {
        // Ask the user which row and column they would want to chomp
        let mut user_move_row: usize = input!("Enter a row for the square you want to remove: ")
            .trim()
            .parse()
            .unwrap();
        let mut user_move_col: usize = input!("Enter a col for the square you want to remove: ")
            .trim()
            .parse()
            .unwrap();
        if user_move_row == 0 && user_move_col == 0 {
            println!("Game Over");
            println!("You Lose");
            break;
        }
        while Board::check_user_input(&board, user_move_row, user_move_col) == false {
            user_move_row =
                input!("Invalid Input!!! Enter a row for the square you want to remove: ")
                    .trim()
                    .parse()
                    .unwrap();
            user_move_col =
                input!("Invalid Input!!! Enter a col for the square you want to remove: ")
                    .trim()
                    .parse()
                    .unwrap();
            if user_move_row == 0 && user_move_col == 0 {
                println!("Game Over");
                println!("You Lose");
                break 'outer;
            }
        }
        Board::chomp(&mut board, user_move_row, user_move_col);
        Board::print_board(&board);

        // Try to find a winning move. If there is one, chomp the winning_move
        if let Some(winning_move) = Board::winning_move(&board) {
            println!(
                "The winning move is: ({}, {})",
                winning_move.0, winning_move.1
            );
            Board::chomp(&mut board, winning_move.0, winning_move.1);
        } else {
            Board::chomp_furthest_right(&mut board); // Otherwise, stall by chomping as little as possible
        }

        // display the board again after the AI played
        Board::print_board(&board);
    }
}
