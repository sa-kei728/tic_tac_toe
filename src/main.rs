use tic_tac_toe::board::Board;
use tic_tac_toe::board::Turn;
use tic_tac_toe::piece::Piece;
use tic_tac_toe::player::*;

fn main() {
    const BOARD_SIZE: usize = 3;

    //board setup
    let player1 = Human::new(Piece::Nought);
    let player2 = Human::new(Piece::Cross);
    let mut game = Board::new(BOARD_SIZE, player1, player2);
    game.print();

    'game: loop {
        for turn in vec![Turn::First, Turn::Second] {
            game.set_piece(turn);
            game.print();
            if let Some(s) = game.judge() {
                println!("{}", s);
                break 'game;
            }
        }
    }
}
