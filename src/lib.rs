pub const MAX_WIDTH: usize = 5;
pub const MAX_HEIGHT: usize = 4;

#[derive(Clone)]
pub struct Board {
    width: usize,
    height: usize,
    squares: [[bool; MAX_WIDTH]; MAX_HEIGHT],
}

impl Board {
    // function to create a board with a given width and height
    pub fn create_board(width: usize, height: usize) -> Self {
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

    // Function to chomp the furthest right piece in the lowermost nonempty row
    // If no winning move is found
    pub fn chomp_furthest_right(&mut self) {
        'outer: for r in (0..self.height).rev() {
            for c in (0..self.width).rev() {
                if self.squares[r][c] {
                    self.squares[r][c] = false;
                    break 'outer;
                }
            }
        }
    }

    pub fn winning_move(&self) -> Option<(usize, usize)> {
        // Check whether the board state is already lost.
        //If so, then there is no winning move
        if self.is_game_over() {
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
                    if new_board.winning_move().is_none() {
                        return Some((r, c));
                    }
                }
            }
        }
        None
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
        false
    }

    pub fn check_user_input(&self, row: &str, col: &str) -> Option<(usize, usize)> {
        // mentioning the type <usize> to parse because parse has one input type but many output types so we have to
        // explicitly mention what type we want
        // "collect" method also has similar problem as "parse"
        let row = row.trim().parse::<usize>().ok()?;
        let col = col.trim().parse::<usize>().ok()?;
        if row < self.height && col < self.width && self.squares[row][col] {
            return Some((row, col));
        }
        None
    }
}

// testing my chomp game
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board() {
        let board = Board::create_board(5, 4);
        assert_eq!(board.width, 5);
        assert_eq!(board.height, 4);
    }

    #[test]
    fn test_chomp() {
        let mut board = Board::create_board(5, 4);
        board.chomp(3, 4);
        assert_eq!(board.squares[3][4], false);
    }

    #[test]
    fn test_winning_move() {
        let mut board = Board::create_board(4, 4);
        board.chomp(0, 1);
        let winning_move = board.winning_move();
        assert_eq!(winning_move, Some((1, 0)));
    }

    #[test]
    fn test_check_user_input() {
        let mut board = Board::create_board(3, 3);

        // Test valid user input
        let valid_input = board.check_user_input("1", "2");
        assert_eq!(valid_input, Some((1, 2)));

        // Test invalid user input (out of bounds)
        let invalid_input = board.check_user_input("3", "2");
        assert_eq!(invalid_input, None);

        board.chomp(1, 1);
        //Test to check if the square that is alr eaten is an invlid input
        let invalid_input = board.check_user_input("1", "1");
        assert_eq!(invalid_input, None);
    }
}
