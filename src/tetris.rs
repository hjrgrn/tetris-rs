//! TODO: merges the logic from ui and backend (they are independent)

use std::io::{self, Stdout, Write};

use crossterm::{ExecutableCommand, terminal};

use crate::ui::Container;

pub fn game() -> io::Result<()> {
    let mut tg = TetrisGame::new(40, 40);
    let initial_move = Move::None;

    loop {
        tg.draw()?;
        if tg.tick(&initial_move) {
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
    // XXX:
    cleared_lines: u8,
}

/// XXX: A tetris move.
pub enum Move {
    Left,
    Right,
    Clock,
    Counter,
    Drop,
    Hold,
    None,
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
            cleared_lines: 0,
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

    pub fn tick(&mut self, mv: &Move) -> bool {
        // Handle gravity.
        self.gravity_tick();
        // Handle input.
        self.handle_move(mv);
        // Check for cleared lines.
        self.check_cleared_lines();
        // Adjust score.
        self.adjust_score();
        // Return whether the game is over.
        self.game_over()
    }

    /// TODO:
    fn gravity_tick(&self) {}

    /// TODO:
    fn handle_move(&self, _mv: &Move) {}

    /// TODO:
    fn check_cleared_lines(&mut self) {
        self.cleared_lines = 0;
    }

    /// TODO:
    fn adjust_score(&self) {
        // XXX: adjust score based on self.cleared_lines
    }

    /// TODO:
    fn game_over(&self) -> bool {
        true
    }
}
