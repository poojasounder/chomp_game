extern crate chomp_game;
extern crate prompted;

use chomp_game::Board;
use chomp_game::MAX_HEIGHT;
use chomp_game::MAX_WIDTH;
use prompted::input;

fn main() {
    println!("Welcome to my chomp game");
    println!("Let's create your custom board!");

    // read user input for their desired board size
    let mut user_width = input!("Enter a width for the board: ")
        .trim()
        .parse()
        .unwrap();
    let mut user_height = input!("Enter a height for the board: ")
        .trim()
        .parse()
        .unwrap();

    while user_width > MAX_WIDTH || user_height > MAX_HEIGHT {
        println!();
        println!("Invalid width or height!!!");
        println!("Width must be lesser than or equal to {} AND Height must be lesser than or equal to {}", MAX_WIDTH,MAX_HEIGHT);

        user_width = input!("Enter a width for the board: ")
            .trim()
            .parse()
            .unwrap();
        user_height = input!("Enter a height for the board: ")
            .trim()
            .parse()
            .unwrap();
    }

    // Create the board of the desired width and height
    let mut board = Board::create_board(user_width, user_height);

    // display the board
    board.print_board();

    // Run this loop till game is over
    while board.is_game_over() == false {
        // loop through till the user enters the correct row and column
        let (user_move_row, user_move_col) = loop {
            let user_move_row = input!("Enter a row for the square you want to remove: ");
            let user_move_col = input!("Enter a col for the square you want to remove: ");
            match board.check_user_input(&user_move_row, &user_move_col) {
                Some((user_move_row, user_move_col)) => break (user_move_row, user_move_col),
                None => continue,
            };
        };

        // if the user eats the poisonous square, then they lose
        if user_move_row == 0 && user_move_col == 0 {
            println!("Game Over");
            println!("You Lose");
            break;
        }
        board.chomp(user_move_row, user_move_col);
        board.print_board();

        if board.is_game_over() {
            println!("You Win");
            break;
        }
        // Try to find a winning move. If there is one, chomp the winning_move
        if let Some(winning_move) = board.winning_move() {
            println!(
                "The winning move is: ({}, {})",
                winning_move.0, winning_move.1
            );
            board.chomp(winning_move.0, winning_move.1);
        } else {
            board.chomp_furthest_right(); // Otherwise, stall by chomping as little as possible
        }

        println!();
        // display the board again after the AI played
        board.print_board();
    }
}
