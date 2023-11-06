# Name: Pooja Sounded Rajan
## Chomp

I implemented my version of chomp game and I implemented negamax to find the winning_move for the AI.
I wrote a library crate that writes functions to create the board, display the board, implement the chomp
effect, figure out if game is over and the winning_move function. I implemented a binary crate where I run my program
until the game is over. The board state is considered lost only is the upper left square is the only one left.

The implementation of the game went well. I received an error of
"thread 'main' has overflowed its stack
fatal runtime error: stack overflow"
I resolved the error by not doing my recursive calls extensively and
there was a problem with my winning_move function which caused that error.

I implemented unit test for my create_board, winning_move and chomp functions.

To run my program:
Simply enter "cargo run"

*** My row and column index starts from 0 ***

TODO
-----> Unit tesing for the functions

