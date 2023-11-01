pub struct Board{
    width: usize,
    height: usize,
    squares: Vec<Vec<bool>>,
}

impl Board{

    pub fn create_board(width: usize, height:usize) -> Self{
        let mut squares = vec![vec![true;height];width];
        Board{
            width,
            height,
            squares,
        }
    }

    pub fn print_board(board: &Board){
        for row in 0..board.width{
            for col in 0..board.height{
                print!("{} ",board.squares[row][col]);
            }
            println!()
        }
    }
}
