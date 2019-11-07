use std::fmt::Display;
use crate::piece::Piece;
use crate::player::*;

pub enum Turn{
    First,
    Second,
}

pub enum JudgeResult{
    WinP1,
    WinP2,
    Draw,
}
impl Default for JudgeResult {
    fn default() -> Self {
        JudgeResult::Draw
    }
}
impl Display for JudgeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match *self {
            JudgeResult::WinP1 => write!(f, "{}", "Player1 Win!"),
            JudgeResult::WinP2 => write!(f, "{}", "Player2 Win!"),
            JudgeResult::Draw  => write!(f, "{}", "draw!"),
        }
    }
}

pub struct Board<T: Player> {
    board_size: usize,
    board_matrix: Vec<Vec<Piece>>,
    first_turn: T,
    second_turn: T,
}

impl<T:Player> Board<T> {
    // Constructer
    pub fn new(size: usize, player1: T, player2: T) -> Self {
        println!("OX game");

        let mut v = Vec::new(); // board base vector
        // create vec![[vec!; size]; size]
        for _ in 0..size {
            let v_line = Vec::new();
            v.push(v_line);
        }
        for index in 0..size {
            for _ in 0..size {
                v[index].push(Piece::default());
            }
        }

        Board{
            board_size: size, 
            board_matrix: v, 
            first_turn: player1, 
            second_turn: player2, 
        }
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
                s += &board_matrix[*row][col].to_string();
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

        // print function start.
        printheader(self.board_size);

        let mut index_vec = Vec::new();
        for i in 0..self.board_size {
            index_vec.push(i);
        }
        let index_iter = index_vec.iter();
        for index in index_iter {
            print_pieceline(index, self.board_size, &self.board_matrix);
            if *index != (self.board_size - 1) {
                printline(self.board_size);
            }
        }
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
        fn piece2judge(piece: &Piece) -> Result<JudgeResult,()> {
            match piece{
                Piece::Nought => Ok(JudgeResult::WinP1),
                Piece::Cross  => Ok(JudgeResult::WinP2),
                _ => Err(()),
            }
        }

        let mut is_space = false;
        for diag_i in 0..self.board_size {
            let src_piece = &self.board_matrix[diag_i][diag_i];
            if *src_piece != Piece::Space {
                //row_check
                for col in (diag_i + 1)..self.board_size {
                    let check_piece = &self.board_matrix[diag_i][col];
                    if *src_piece == *check_piece{
                        if col == (self.board_size - 1) {
                            return piece2judge(src_piece).ok();
                        } else {
                            continue
                        }
                    }else if Piece::Space == *check_piece{
                        is_space = true;
                        break;
                    }else{
                        break;
                    }
                }
                //col_check
                for row in (diag_i + 1)..self.board_size {
                    let check_piece = &self.board_matrix[row][diag_i];
                    if *src_piece == *check_piece{
                        if row == (self.board_size - 1){
                            return piece2judge(src_piece).ok();
                        } else {
                            continue
                        }
                    }else if Piece::Space == *check_piece{
                        is_space = true;
                        break;
                    }else{
                        break;
                    }
                }
            }
        }
        //right_diag_check
        let src_piece = &self.board_matrix[0][0];
        for diag in 1..self.board_size {
            let check_piece = &self.board_matrix[diag][diag];
            if *src_piece == *check_piece{
                if diag == (self.board_size - 1) {
                    return piece2judge(src_piece).ok();
                } else {
                    continue
                }
            }else if Piece::Space == *check_piece{
                is_space = true;
                break;
            }else{
                break;
            }
        }
        //left_diag_check
        let src_piece = &self.board_matrix[self.board_size - 1][0];
        for diag in 1..self.board_size {
            let check_piece = &self.board_matrix[diag][(self.board_size - 1) - diag];
            if *src_piece == *check_piece{
                if diag == (self.board_size - 1) {
                    return piece2judge(src_piece).ok();
                } else {
                    continue
                }
            }else if Piece::Space == *check_piece{
                is_space = true;
                break;
            }else{
                break;
            }
        }

        if is_space == true {
            None
        } else {
            Some(JudgeResult::Draw)
        }     
    }
}
