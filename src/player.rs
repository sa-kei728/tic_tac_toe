use crate::piece::Piece;
use crate::operator::*;

// create player trait better than struct
pub struct Player {
    operator_type: Operator,
    piece_type: Piece,
}

impl Player {
    // Constructer
    pub fn new(op: Operator, piece: Piece) -> Self {
        Player{operator_type: op, piece_type: piece}
    }

    // return Tuple(row, col, Piece)
    pub fn select_matrix(&self, max_size: usize) -> (usize, usize) {
        //input from stdin
        println!("row:");
        let row : usize = Player::read_stdin(max_size);

        println!("col:");
        let col : usize = Player::read_stdin(max_size);
        (row, col)
    }

    fn read_stdin(max_size: usize) -> usize {
        let select_area : usize = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).ok();
            s.trim().parse().ok().unwrap()
        };
        if select_area < max_size {
            select_area
        }else{
            println!("out of range area! reinput please!");
            Player::read_stdin(max_size)
        }
    }

    pub fn get_piece_type(&self) -> Piece {
        let piece_type = self.piece_type.clone();
        piece_type
    }
}
