//! TODO: merges the logic from ui and backend (they are independent)

use std::io::{self, Write};

use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{self, Stylize},
    terminal,
};

use crate::{
    backend::{Move, TetrisGame, tick},
    ui::Container,
};

pub fn game() -> io::Result<()> {
    let tg = TetrisGame {};
    let initial_move = Move {};

    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    let mut board = Container::new(40, 150, 0, 0);
    board.draw(&mut stdout)?;
    stdout.flush()?;

    loop {
        if tick(&tg, &initial_move) {
            break;
        }
    }

    Ok(())
}
