//! XXX:

use crate::tetris::TETRIS;

/// A a struct that represents a
/// [tetromino](https://en.wikipedia.org/wiki/Tetromino).
///
/// Specifically: its type, its orientation, and where it is.
pub struct TetrisBlock {
    ty: BlockType,
    orientation: Orientation,
    location: Location,
    cells: [Location; TETRIS],
}

impl TetrisBlock {
    pub fn new(col: i8) -> Self {
        let ty: BlockType = rand::random();
        let orientation: Orientation = rand::random();
        let location = Location {
            row: 0,
            col: col / 2 - 2,
        };
        let cells = gen_cells(&orientation, &ty);
        Self {
            ty,
            orientation,
            location,
            cells,
        }
    }
}

/// `TetrisBlock`'s helper function.
///
/// Generate the appropriate set of cells based on the orientation and the type
/// of the block.
fn gen_cells(orientation: &Orientation, ty: &BlockType) -> [Location; TETRIS] {
    match ty {
        BlockType::I => match orientation {
            Orientation::Zero => [
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
                Location { row: 1, col: 3 },
            ],
            Orientation::One => [
                Location { row: 0, col: 2 },
                Location { row: 1, col: 2 },
                Location { row: 2, col: 2 },
                Location { row: 3, col: 2 },
            ],
            Orientation::Two => [
                Location { row: 3, col: 0 },
                Location { row: 3, col: 1 },
                Location { row: 3, col: 2 },
                Location { row: 3, col: 3 },
            ],
            Orientation::Three => [
                Location { row: 0, col: 1 },
                Location { row: 1, col: 1 },
                Location { row: 2, col: 1 },
                Location { row: 3, col: 1 },
            ],
        },
        BlockType::J => match orientation {
            Orientation::Zero => [
                Location { row: 0, col: 0 },
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
            ],
            Orientation::One => [
                Location { row: 0, col: 1 },
                Location { row: 0, col: 2 },
                Location { row: 1, col: 1 },
                Location { row: 2, col: 1 },
            ],
            Orientation::Two => [
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
                Location { row: 2, col: 2 },
            ],
            Orientation::Three => [
                Location { row: 0, col: 1 },
                Location { row: 1, col: 1 },
                Location { row: 2, col: 0 },
                Location { row: 2, col: 1 },
            ],
        },
        BlockType::L => match orientation {
            Orientation::Zero => [
                Location { row: 0, col: 2 },
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
            ],
            Orientation::One => [
                Location { row: 0, col: 1 },
                Location { row: 1, col: 1 },
                Location { row: 2, col: 1 },
                Location { row: 2, col: 2 },
            ],
            Orientation::Two => [
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
                Location { row: 2, col: 0 },
            ],
            Orientation::Three => [
                Location { row: 0, col: 0 },
                Location { row: 0, col: 1 },
                Location { row: 1, col: 1 },
                Location { row: 2, col: 1 },
            ],
        },
        BlockType::O => match orientation {
            Orientation::Zero => [
                Location { row: 0, col: 1 },
                Location { row: 0, col: 2 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
            ],
            Orientation::One => [
                Location { row: 0, col: 1 },
                Location { row: 0, col: 2 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
            ],
            Orientation::Two => [
                Location { row: 0, col: 1 },
                Location { row: 0, col: 2 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
            ],
            Orientation::Three => [
                Location { row: 0, col: 1 },
                Location { row: 0, col: 2 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
            ],
        },
        BlockType::S => match orientation {
            Orientation::Zero => [
                Location { row: 0, col: 1 },
                Location { row: 0, col: 2 },
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
            ],
            Orientation::One => [
                Location { row: 0, col: 1 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
                Location { row: 2, col: 2 },
            ],
            Orientation::Two => [
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
                Location { row: 2, col: 0 },
                Location { row: 2, col: 1 },
            ],
            Orientation::Three => [
                Location { row: 0, col: 0 },
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 2, col: 1 },
            ],
        },
        BlockType::T => match orientation {
            Orientation::Zero => [
                Location { row: 0, col: 1 },
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
            ],
            Orientation::One => [
                Location { row: 0, col: 1 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
                Location { row: 2, col: 1 },
            ],
            Orientation::Two => [
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
                Location { row: 2, col: 1 },
            ],
            Orientation::Three => [
                Location { row: 0, col: 1 },
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 2, col: 1 },
            ],
        },
        BlockType::Z => match orientation {
            Orientation::Zero => [
                Location { row: 0, col: 0 },
                Location { row: 0, col: 1 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
            ],
            Orientation::One => [
                Location { row: 0, col: 2 },
                Location { row: 1, col: 1 },
                Location { row: 1, col: 2 },
                Location { row: 2, col: 1 },
            ],
            Orientation::Two => [
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 2, col: 1 },
                Location { row: 2, col: 2 },
            ],
            Orientation::Three => [
                Location { row: 0, col: 1 },
                Location { row: 1, col: 0 },
                Location { row: 1, col: 1 },
                Location { row: 2, col: 0 },
            ],
        },
    }
}

/// The type/shape of a tetromino, not including orientation.
enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Distribution<BlockType> for StandardUniform {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        match rng.random_range(0..7) {
            0 => BlockType::I,
            1 => BlockType::O,
            2 => BlockType::T,
            3 => BlockType::L,
            4 => BlockType::J,
            5 => BlockType::S,
            _ => BlockType::Z,
        }
    }
}

/// The orientation of a tetromino.
enum Orientation {
    Zero,
    One,
    Two,
    Three,
}

impl Distribution<Orientation> for StandardUniform {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Orientation {
        match rng.random_range(0..4) {
            0 => Orientation::Zero,
            1 => Orientation::One,
            2 => Orientation::Two,
            _ => Orientation::Three,
        }
    }
}

/// A row,column pair, representing a location on the board. Negative numbers
/// are allowed, since we need them for offsets.
struct Location {
    row: i8,
    col: i8,
}
