use crate::tetris::TetrisGame;

/// TODO: return true if game is over
pub fn tick(_tg: &TetrisGame, _mv: &Move) -> bool {
    true
}

/// XXX: A tetris move
pub enum Move {
    Left,
    Right,
    Clock,
    Counter,
    Drop,
    Hold,
    None,
}
