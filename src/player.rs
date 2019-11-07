use std::io::{stdout, Write};
use crate::piece::Piece;

pub trait Player{
    fn new(piece: Piece) -> Self;
    fn select_matrix(&self, max_size: usize) -> (usize, usize);
    fn get_piece_type(&self) -> Piece;
}

// create player trait better than struct
pub struct Human {
    piece_type: Piece,
}

impl Player for Human{
    // Constructer
    fn new(piece: Piece) -> Self {
        Human{piece_type: piece}
    }

    // return Tuple(row, col, Piece)
    fn select_matrix(&self, max_size: usize) -> (usize, usize) {
        //input from stdin
        print!("row:");
        stdout().flush().unwrap();
        let row : usize = Human::read_stdin(max_size);

        print!("col:");
        stdout().flush().unwrap();
        let col : usize = Human::read_stdin(max_size);
        (row, col)
    }
    
    fn get_piece_type(&self) -> Piece {
        let piece_type = self.piece_type.clone();
        piece_type
    }
}

impl Human {
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
            Human::read_stdin(max_size)
        }
    }
}
