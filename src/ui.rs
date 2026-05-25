use std::io::{self, Stdout};

use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
};

use crate::tetris::block::{Location, TetrisBlock};

pub struct Container {
    rows: u16,
    cols: u16,
    start_y: u16,
    start_x: u16,
    board: Vec<char>,
}

impl Container {
    pub fn new(rows: u16, cols: u16, start_y: u16, start_x: u16) -> Self {
        // TODO: error handling
        let mut board = Vec::with_capacity((rows * cols).try_into().unwrap());
        for y in start_y..rows + start_y {
            for x in start_x..cols + start_x {
                let is_top = y == start_y;
                let is_bottom = y == rows + start_y - 1;
                let is_left = x == start_x;
                let is_right = x == cols + start_x - 1;
                let c = if is_top && is_left {
                    '┏'
                } else if is_top && is_right {
                    '┓'
                } else if is_bottom && is_left {
                    '┗'
                } else if is_bottom && is_right {
                    '┛'
                } else if is_bottom || is_top {
                    '━'
                } else if is_left || is_right {
                    '┃'
                } else {
                    continue;
                };
                // TODO: error handling
                board[<u16 as Into<usize>>::into(y * (start_y + rows) + x + start_x)] = c;
            }
        }
        Self {
            rows,
            cols,
            start_y,
            start_x,
            board,
        }
    }

    pub fn draw(&mut self, stdout: &mut Stdout) -> io::Result<()> {
        // TODO: different colors for different elements
        for y in self.start_y..self.rows + self.start_y {
            for x in self.start_x..self.cols + self.start_x {
                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(
                        self.board[<u16 as Into<usize>>::into(
                            y * (self.start_y + self.rows) + x + self.start_x,
                        )]
                        .magenta(),
                    ))?;
            }
        }
        Ok(())
    }

    /// Set the block at the given row and column.
    pub fn set(&mut self, block: &TetrisBlock, val: char) {
        // obj->board[obj->cols * row + column] = value;
        // TODO: edge cases and error handling.
        // TODO: testing.
        for cell in block.cells.iter() {
            let x = block.location.col + cell.col;
            let y = block.location.row + cell.row;
            self.board[self.cols as usize * y as usize + x as usize] = val;
        }
    }

    /// Get the value of a cell.
    pub fn get(&self, location: &Location) -> char {
        // TODO: edge cases and error handling.
        // TODO: testing.
        self.board[self.cols as usize * location.row as usize + location.row as usize]
    }
}
