//! TODO: merges the logic from ui and backend (they are independent)

use std::io::{self, Stdout, Write};

use crossterm::{ExecutableCommand, terminal};

use crate::ui::Container;

static GRAVITY: u8 = 50;

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

/// A a struct that represents a
/// [tetromino](https://en.wikipedia.org/wiki/Tetromino).
///
/// Specifically: its type, its orientation, and where it is.
struct TetrisBlock {
    ty: BlockType,
    orientation: Orientation,
    location: Location,
}

/// The type/shape of a tetromino, not including orientation.
enum BlockType {
    I,
    O,
    T,
    L,
    J,
    S,
    Z,
}

/// The orientation of a tetromino.
enum Orientation {
    Zero,
    One,
    Two,
    Three,
}

/// A row,column pair, representing a location on the board. Negative numbers
/// are allowed, since we need them for offsets.
struct Location {
    row: i8,
    col: i8,
}

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

// TODO: separate ui and game fields in different structs, move those structs into different
// modules.
pub struct TetrisGame {
    col: u16,
    row: u16,
    board: Container,
    next: Container,
    hold: Container,
    stdout: Stdout,
    // XXX:
    cleared_lines: u8,
    /// Number of game ticks until the block will touch the ground.
    ticks_till_ground: u8,
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
            // TODO: different gravity levels
            ticks_till_ground: GRAVITY,
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
    fn gravity_tick(&mut self) {}

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
