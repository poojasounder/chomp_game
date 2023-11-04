const MAX_WIDTH: usize = 5;
const MAX_HEIGHT: usize = 4;

#[derive(Clone)]
pub struct Board {
    width: usize,
    height: usize,
    squares: [[bool; MAX_WIDTH]; MAX_HEIGHT],
}

impl Board {
    // function to create a board with a given width and height
    pub fn create_board(width: usize, height: usize) -> Self {
        assert!(
            width <= MAX_WIDTH && height <= MAX_HEIGHT,
            "Width must be lesser than or equal to 4 and Height must be lesser than or equal to 5"
        );

        // setting all the squares to true initially
        let squares = [[true; MAX_WIDTH]; MAX_HEIGHT];
        Board {
            width,
            height,
            squares,
        }
    }

    // function to dispplay the board
    pub fn print_board(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let symbol = if self.squares[row][col] { 'X' } else { ' ' };
                print!("{}", symbol);
            }
            println!();
        }
    }

    // Chomp a given square, removing all squares below it
    // and to the right of it
    pub fn chomp(&mut self, row: usize, col: usize) {
        for r in row..self.height {
            for c in col..self.width {
                self.squares[r][c] = false;
            }
        }
    }

    pub fn chomp_furthest_right(&mut self){
        'outer:for r in (0..self.height).rev(){
            for c in (0..self.width).rev(){
                println!("{}",r);
                println!("{}",c);
                if self.squares[r][c]{
                    println!("hi");
                    self.squares[r][c] = false;
                    break 'outer;
                }
            }
        }
    }
    pub fn winning_move(&self) -> Option<(usize, usize)> {
        // Check whether the board state is already lost.
        //If so, then there is no winning move
        if self.is_game_over() == true {
            return None;
        }

        // Otherwise for each possible move
        for r in 0..self.height {
            for c in 0..self.width {
                if r == 0 && c == 0 {
                    continue;
                }
                if self.squares[r][c] {
                    // Create a new_board
                    let mut new_board = self.clone();
                    //Perform the move on new_board
                    new_board.chomp(r, c);
                    //Call winning_move recursively at new_board and
                    //if winning_move outputs a winning_move for new_board
                    //then this move is not a winning_move . Continue to the next move
                    //Otherwise, this is the winning move and return it
                    if new_board.winning_move() == None {
                        return Some((r, c));
                    }
                }
            }
        }
        return None;
    }

    // Function to check if the game is over.
    // the board state is lost if the upper-left square is the only one left
    pub fn is_game_over(&self) -> bool {
        // total number of squares in the board
        let mut count = self.width * self.height;
        for i in 0..self.height {
            for j in 0..self.width {
                // if the square is labeled as false ---> eaten
                if !self.squares[i][j] {
                    count -= 1;
                }
            }
        }

        // if there is only one square left and it is the upper-left square,
        // return true else, return false
        if count == 1 && self.squares[0][0] {
            return true;
        }
        return false;
    }
}
