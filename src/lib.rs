
const MAX_WIDTH: usize = 5;
const MAX_HEIGHT: usize = 4;

#[derive(Clone)]
pub struct Board{
    width: usize,
    height: usize,
    squares: [[bool; MAX_WIDTH]; MAX_HEIGHT],
}

impl Board{

    pub fn create_board(width: usize, height:usize) -> Self{
        assert!(width<=MAX_WIDTH && height <=MAX_HEIGHT, "Width must be lesser than or equal to 4 and Height must be lesser than or equal to 5");
        let squares = [[true;MAX_WIDTH];MAX_HEIGHT];
        Board{
            width,
            height,
            squares,
        }
    }

    pub fn print_board(&self){
        
        for row in 0..self.height {
            for col in 0..self.width {
                let symbol = if self.squares[row][col] {
                    'X'
                } else {
                    ' '
                };
                print!("{}", symbol);
            }
            println!();
        }
    }
    pub fn chomp(&mut self,row:usize, col: usize){
        if row < self.width && col < self.height{
            for r in row..self.height{
                for c in col..self.width{
                    self.squares[r][c] = false;
                }
            }
        }
        else if row< self.width && col >= self.height{
            for r in row..self.width{
                self.squares[r][col] = false;
            }
        }
        else{
            for c in col..self.height{
                self.squares[row][c] = false;
            }
        }
    }

    
    pub fn winning_move(&self){
        for r in 0..self.height{
            for c in 0..self.width{
                if r == 0 && c==0{
                    continue;
                }
                let mut new_board = self.clone();
                new_board.chomp(r,c);
            }
        }
    }

}



