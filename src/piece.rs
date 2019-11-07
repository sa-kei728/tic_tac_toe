use std::fmt::Display;

#[derive(PartialEq, Clone)]
pub enum Piece {
    Space,
    Nought,  //Use O instead of 〇
    Cross,   //Use X instead of ×
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Space
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match *self {
            Piece::Space => write!(f, "{}", " "),
            Piece::Nought => write!(f, "{}", "O"),
            Piece::Cross  => write!(f, "{}", "X"),
        }
    }
}

