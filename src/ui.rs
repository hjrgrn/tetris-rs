use std::io::{self, Stdout};

use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
};

pub struct Container {
    row: u16,
    col: u16,
    start_y: u16,
    start_x: u16,
}

impl Container {
    pub fn new(row: u16, col: u16, start_y: u16, start_x: u16) -> Self {
        Self {
            row,
            col,
            start_y,
            start_x,
        }
    }

    pub fn draw(&mut self, stdout: &mut Stdout) -> io::Result<()> {
        for y in self.start_y..self.row + self.start_y {
            for x in self.start_x..self.col + self.start_x {
                let is_top = y == self.start_y;
                let is_bottom = y == self.row + self.start_y - 1;
                let is_left = x == self.start_x;
                let is_right = x == self.col + self.start_x - 1;
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
