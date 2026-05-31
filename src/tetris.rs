//! TODO: merges the logic from ui and backend (they are independent)

use std::io::{self, Stdout, Write};

use crossterm::{ExecutableCommand, terminal};

use crate::{
    tetris::block::{Location, TetrisBlock},
    ui::Container,
};

pub mod block;

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
    cols: u16,
    rows: u16,
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
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            cols,
            rows,
            board: Container::new(cols, rows, 0, 0),
            next: Container::new(rows / 4, cols / 2, 0, cols + 2),
            hold: Container::new(rows / 4, cols / 2, (rows / 4) + 2, cols + 2),
            stdout: io::stdout(),
            cleared_lines: 0,
            // TODO: different gravity levels
            ticks_till_ground: GRAVITY,
            // TODO: solve these `unwrap`s
            falling_block: TetrisBlock::new(cols.try_into().unwrap()),
            next_block: TetrisBlock::new(cols.try_into().unwrap()),
            stored_block: TetrisBlock::new(cols.try_into().unwrap()),
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

    /// Tick gravity, and move the block down if gravity should act.
    // NOTE: this hasn't been tested yet.
    fn gravity_tick(&mut self) {
        self.ticks_till_ground -= 1;
        if self.ticks_till_ground <= 0 {
            self.remove_falling();
            // TODO: this may be a function/method.
            self.falling_block.location.row += 1;
            if self.falling_fits() {
                self.ticks_till_ground = GRAVITY;
            } else {
                self.falling_block.location.row -= 1;
                self.put_falling();
                self.new_falling();
            }
            self.put_falling();
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
    // TODO: this and other `*falling*` methods need a structural refactoring.
    fn remove_falling(&mut self) {
        // TODO: define an enum (search `} tetris_cell;`).
        self.board.set(&self.falling_block, '0');
    }

    /// Check if a block can be placed on the board.
    fn falling_fits(&self) -> bool {
        for cell in self.falling_block.cells.iter() {
            let col = self.falling_block.location.col + cell.col;
            let row = self.falling_block.location.row + cell.row;
            let location = Location { row, col };
            if !self.check(&location) || !self.location_is_empty(&location) {
                return false;
            }
        }
        true
    }

    /// Place a block onto the board.
    fn put_falling(&mut self) {
        self.board.set(
            &self.falling_block,
            (u8::from(&self.falling_block.ty) + 1) as char,
        );
    }

    /// Check whether a row and column are in bounds.
    fn check(&self, location: &Location) -> bool {
        0 <= location.row
            && location.row < self.rows as i8
            && 0 <= location.col
            && location.col < self.cols as i8
    }

    /// Check whether the location provided is empty on the board.
    fn location_is_empty(&self, location: &Location) -> bool {
        // TODO: define an enum (search `} tetris_cell;`).
        self.board.get(location) == '0'
    }

    /// Create a new falling block and populate the next falling block with a random
    /// one.
    fn new_falling(&mut self) {
        // TODO: solve memcopy
        self.falling_block = self.next_block.clone();
        self.next_block = TetrisBlock::new(self.cols.try_into().unwrap());
    }
}
