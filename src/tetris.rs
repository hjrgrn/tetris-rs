//! TODO: merges the logic from ui and backend (they are independent)

use std::io::{self, Stdout, Write};

use crossterm::{ExecutableCommand, terminal};
use rand::{
    RngExt,
    distr::{Distribution, StandardUniform},
};

use crate::{tetris::block::TetrisBlock, ui::Container};

mod block;

static GRAVITY: u8 = 50;
static TETRIS: usize = 4;

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
    /// Falling block.
    falling_block: TetrisBlock,
    /// Next block.
    next_block: TetrisBlock,
    /// A stored block that the player can swap out.
    stored_block: TetrisBlock,
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
            // TODO: solve these `unwrap`s
            falling_block: TetrisBlock::new(col.try_into().unwrap()),
            next_block: TetrisBlock::new(col.try_into().unwrap()),
            stored_block: TetrisBlock::new(col.try_into().unwrap()),
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
        // FROMHERE:
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
    fn gravity_tick(&mut self) {
        self.ticks_till_ground -= 1;
        if self.ticks_till_ground <= 0 {
            // FROMHERE:: remove
        }
    }

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

    /// Clear a block out of the board.
    fn remove(&mut self, block: TetrisBlock) {
        for i in 0..TETRIS {
            // TODOFIRST:
        }
    }
}
