//! TODO: merges the logic from ui and backend (they are independent)

use crate::backend::{Move, TetrisGame, tick};

pub fn game() {
    let tg = TetrisGame {};
    let initial_move = Move {};
    loop {
        if tick(&tg, &initial_move) {
            break;
        }
    }
}
