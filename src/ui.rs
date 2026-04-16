use std::io::{self, Stdout};

use crossterm::{
    QueueableCommand, cursor,
    style::{self, Stylize},
};

pub struct Container {
    col: u16,
    row: u16,
    start_y: u16,
    start_x: u16,
}

impl Container {
    pub fn new(col: u16, row: u16, start_y: u16, start_x: u16) -> Self {
        Self {
            col,
            row,
            start_y,
            start_x,
        }
    }
    // TODO: consider start and end
    pub fn draw(&mut self, stdout: &mut Stdout) -> io::Result<()> {
        for y in self.start_y..self.col + self.start_y {
            for x in self.start_x..self.row + self.start_x {
                let is_top = y == self.start_y;
                let is_bottom = y == self.col + self.start_y - 1;
                let is_left = x == self.start_x;
                let is_right = x == self.row + self.start_x - 1;
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
