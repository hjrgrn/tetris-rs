use std::io::{self, Stdout};

use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
};

pub struct Container {
    col: u16,
    row: u16,
    start: u16,
    end: u16,
}

impl Container {
    pub fn new(col: u16, row: u16, start: u16, end: u16) -> Self {
        Self {
            col,
            row,
            start,
            end,
        }
    }
    // TODO: consider start and end
    pub fn draw(&mut self, stdout: &mut Stdout) -> io::Result<()> {
        for y in 0..self.col {
            for x in 0..self.row {
                let is_top = y == 0;
                let is_bottom = y == self.col - 1;
                let is_left = x == 0;
                let is_right = x == self.row - 1;
                let char = if is_top && is_left {
                    "┏"
                } else if is_top && is_right {
                    "┓"
                } else if is_bottom && is_left {
                    "┗"
                } else if is_bottom && is_right {
                    "┛"
                } else if is_bottom || is_top {
                    "━"
                } else if is_left || is_right {
                    "┃"
                } else {
                    continue;
                };

                stdout
                    .queue(cursor::MoveTo(x, y))?
                    .queue(style::PrintStyledContent(char.magenta()))?;
            }
        }
        Ok(())
    }
}
