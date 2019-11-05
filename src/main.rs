use tic_tac_toe::board::Board;
use tic_tac_toe::board::Turn;
use tic_tac_toe::board::JudgeResult;
use tic_tac_toe::operator::Operator;

fn main() {
    const BOARD_SIZE: usize = 3;

    println!("OX game");
    //Player Setting
    let player1 = Operator::Human;
    let player2 = Operator::Human;

    //board setup
    let mut game = Board::new(BOARD_SIZE, player1, player2);
    game.print();

    //Judge checker
    let mut judge_result = JudgeResult::Draw;

    loop {
        //Player1
        game.set_piece(Turn::First);
        game.print();

        //Player2
        game.set_piece(Turn::Second);
        game.print();
        break;
    }

    //result
    match judge_result {
        JudgeResult::WinP1  => println!("Player1 Win!"),
        JudgeResult::WinP2  => println!("Player2 Win!"),
        JudgeResult::Draw   => println!("Draw!"),
    }
}
