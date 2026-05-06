//! TODO: merges the logic from ui and backend (they are independent)

use std::io::{self, Stdout, Write};

use crossterm::{ExecutableCommand, terminal};

use crate::{
    backend::{Move, tick},
    ui::Container,
};

pub fn game() -> io::Result<()> {
    let mut tg = TetrisGame::new(40, 40);
    let initial_move = Move::None;

    loop {
        tg.draw()?;
        if tick(&tg, &initial_move) {
            break;
        }
    }

    Ok(())
}

pub struct TetrisGame {
    col: u16,
    row: u16,
    board: Container,
    next: Container,
    hold: Container,
    stdout: Stdout,
}

impl TetrisGame {
    pub fn new(col: u16, row: u16) -> Self {
        Self {
            col,
            row,
            board: Container::new(col, row, 0, 0),
            next: Container::new(row / 4, col / 2, 0, col + 2),
            hold: Container::new(row / 4, col / 2, (row / 4) + 2, col + 2),
            stdout: io::stdout(),
        }
    }

    pub fn draw(&mut self) -> io::Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        self.board.draw(&mut self.stdout)?;
        self.next.draw(&mut self.stdout)?;
        self.hold.draw(&mut self.stdout)?;
        self.stdout.flush()
    }
}
