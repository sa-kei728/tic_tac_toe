use crate::piece::Piece;
use crate::player::Player;
use crate::operator::Operator;

pub enum Turn{
    First,
    Second,
}

pub enum JudgeResult{
    WinP1,
    WinP2,
    Draw,
}

pub struct Board {
    board_size: usize,
    board_matrix: Vec<Vec<Piece>>,
    first_turn: Player,
    second_turn: Player,
}

impl Board {
    // Constructer
    pub fn new(size: usize, first: Operator, second: Operator) -> Self {
        let mut v = Vec::new(); // board base vector
        let player1 = Player::new(first, Piece::Nought);
        let player2 = Player::new(second, Piece::Cross);

        // create vec![[vec!; size]; size]
        for _ in 0..size {
            let v_line = Vec::new();
            v.push(v_line);
        }
        for index in 0..size {
            for _ in 0..size {
                v[index].push(Piece::Space);
            }
        }
        Board{ board_size: size, board_matrix: v, first_turn: player1, second_turn: player2}
    }

    /* print board state
     * Ex. board size is 3
     *   0 1 2
     * 0  | |
     *   -+-+-
     * 1  |O|O
     *   -+-+-
     * 2  |X|X
     */ 
    pub fn print(&self) {
        let size_max = self.board_size;
        let board_matrix = &self.board_matrix;

        Board::printheader(size_max);

        let mut index_vec = Vec::new();
        for i in 0..size_max {
            index_vec.push(i);
        }
        let index_iter = index_vec.iter();
        for index in index_iter {
            Board::print_pieceline(index, size_max, &board_matrix);
            if *index != (size_max - 1) {
                Board::printline(size_max);
            }
        }
    }

    // if col_max is 3, output "  0 1 2"
    fn printheader(col_max: usize){
        let mut s = String::with_capacity(1024);
        s += " ";
        for col in 0..col_max {
            s += " ";
            s += &col.to_string();
        }
        println!("{}", s);
    }

    fn print_pieceline(row: &usize, size_max: usize, board_matrix: &Vec<Vec<Piece>>){
        let mut s = String::with_capacity(1024);
        s += &(row.to_string() + " ");
        for col in 0..size_max {
            //should use Display trait for enum Piece 
            match &board_matrix[*row][col] {
                Piece::Space   => s += " ",
                Piece::Nought  => s += "O",
                Piece::Cross   => s += "X",
            };

            if col != (size_max - 1) {
                s += "|";
            }
        }
        println!("{}", s);
    }

    // if col_max is 3, output "  -+-+-"
    fn printline(col_max: usize){
        let mut s = String::with_capacity(1024);
        s += "  ";
        for col in 0..col_max {
            match col {
                0 => s += "-",
                _ => s += "+-",
            }
        }
        println!("{}", s);
    }

    pub fn set_piece(&mut self, turn: Turn) {
        let size_max = self.board_size;
        let player = match turn{
            Turn::First   =>  &self.first_turn,
            Turn::Second  =>  &self.second_turn,
        };
        let (row, col) = player.select_matrix(size_max);

        if self.board_matrix[row][col] == Piece::Space {
            std::mem::replace(&mut self.board_matrix[row][col], player.get_piece_type());;
        } else {
            println!("other player have put piece this area!");
            Board::set_piece(self, turn)
        }
    }

    pub fn judge(&self) -> Option<JudgeResult> {
        Some(JudgeResult::Draw)
    }
}
