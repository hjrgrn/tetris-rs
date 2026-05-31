//! XXX:

use crate::tetris::TETRIS;

use rand::{
    RngExt,
    distr::{Distribution, StandardUniform},
};

/// A a struct that represents a
/// [tetromino](https://en.wikipedia.org/wiki/Tetromino).
///
/// Specifically: its type, its orientation, and where it is.
#[derive(Clone, Debug)]
pub struct TetrisBlock {
    orientation: Orientation,
    // TODO: make these private, maybe use amplify
    // amplify = { version = "4", default-features = false, features = ["derive"] }
    // IDEA: Now we have Location, that describes the location, and cells, that describe the shape
    // of the Block, maybe we can have just cells, that describes the shape AND the location.
    pub location: Location,
    pub cells: [Location; TETRIS],
    pub ty: BlockType,
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
#[derive(Debug, Clone)]
pub enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TryFrom<u8> for BlockType {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BlockType::I),
            1 => Ok(BlockType::J),
            2 => Ok(BlockType::L),
            3 => Ok(BlockType::O),
            4 => Ok(BlockType::S),
            5 => Ok(BlockType::T),
            6 => Ok(BlockType::Z),
            _ => Err(anyhow::anyhow!("TODO")),
        }
    }
}

impl From<&BlockType> for u8 {
    fn from(value: &BlockType) -> Self {
        match value {
            BlockType::I => 0,
            BlockType::J => 1,
            BlockType::L => 2,
            BlockType::O => 3,
            BlockType::S => 4,
            BlockType::T => 5,
            BlockType::Z => 6,
        }
    }
}

impl Distribution<BlockType> for StandardUniform {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        let i = rng.random_range(0..7);
        BlockType::try_from(i).expect("This will never happen.")
    }
}

/// The orientation of a tetromino.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Location {
    // TODO: make these private, maybe use amplify
    // amplify = { version = "4", default-features = false, features = ["derive"] }
    pub row: i8,
    pub col: i8,
}
