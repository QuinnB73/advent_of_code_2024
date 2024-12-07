use std::fmt;

#[derive(Debug, Clone)]
pub struct PuzzleError {
    pub msg: String,
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed the puzzle! {}", self.msg)
    }
}
