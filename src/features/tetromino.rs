use bevy::prelude::*;

pub struct PieceShape {
    pub offsets: [[IVec2; 4]; 4],
    pub color: Color,
}

pub const TETROMINOES: [Tetromino; 7] = [
    Tetromino::I,
    Tetromino::O,
    Tetromino::T,
    Tetromino::S,
    Tetromino::Z,
    Tetromino::J,
    Tetromino::L,
];

#[derive(Clone, Copy)]
pub enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl Tetromino {
    pub fn shape(&self) -> PieceShape {
        match self {
            Self::I => PieceShape {
                offsets: [
                    [ivec2(0, 2), ivec2(1, 2), ivec2(2, 2), ivec2(3, 2)],
                    [ivec2(2, 0), ivec2(2, 1), ivec2(2, 2), ivec2(2, 3)],
                    [ivec2(0, 1), ivec2(1, 1), ivec2(2, 1), ivec2(3, 1)],
                    [ivec2(1, 0), ivec2(1, 1), ivec2(1, 2), ivec2(1, 3)],
                ],
                color: Color::srgb_u8(49, 199, 239), // Light blue
            },
            Self::O => PieceShape {
                offsets: [[ivec2(0, 0), ivec2(0, 1), ivec2(1, 0), ivec2(1, 1)]; 4],
                color: Color::srgb_u8(247, 211, 8), // Yellow
            },
            Self::T => PieceShape {
                offsets: [
                    [ivec2(0, 1), ivec2(1, 1), ivec2(2, 1), ivec2(1, 2)],
                    [ivec2(1, 0), ivec2(1, 1), ivec2(1, 2), ivec2(2, 1)],
                    [ivec2(1, 0), ivec2(0, 1), ivec2(1, 1), ivec2(2, 1)],
                    [ivec2(0, 1), ivec2(1, 0), ivec2(1, 1), ivec2(1, 2)],
                ],
                color: Color::srgb_u8(179, 77, 156), // Purple
            },
            Self::S => PieceShape {
                offsets: [
                    [ivec2(0, 1), ivec2(1, 1), ivec2(1, 2), ivec2(2, 2)],
                    [ivec2(2, 0), ivec2(1, 1), ivec2(2, 1), ivec2(1, 2)],
                    [ivec2(0, 0), ivec2(1, 0), ivec2(1, 1), ivec2(2, 1)],
                    [ivec2(1, 0), ivec2(0, 1), ivec2(1, 1), ivec2(0, 2)],
                ],
                color: Color::srgb_u8(66, 182, 66), // Green
            },
            Self::Z => PieceShape {
                offsets: [
                    [ivec2(1, 1), ivec2(2, 1), ivec2(0, 2), ivec2(1, 2)],
                    [ivec2(1, 0), ivec2(1, 1), ivec2(2, 1), ivec2(2, 2)],
                    [ivec2(1, 0), ivec2(2, 0), ivec2(0, 1), ivec2(1, 1)],
                    [ivec2(0, 0), ivec2(0, 1), ivec2(1, 1), ivec2(1, 2)],
                ],
                color: Color::srgb_u8(239, 32, 41), // Red
            },
            Self::J => PieceShape {
                offsets: [
                    [ivec2(0, 1), ivec2(1, 1), ivec2(2, 1), ivec2(0, 2)],
                    [ivec2(1, 0), ivec2(1, 1), ivec2(1, 2), ivec2(2, 2)],
                    [ivec2(2, 0), ivec2(0, 1), ivec2(1, 1), ivec2(2, 1)],
                    [ivec2(0, 0), ivec2(1, 0), ivec2(1, 1), ivec2(1, 2)],
                ],
                color: Color::srgb_u8(90, 101, 173), // Blue
            },
            Self::L => PieceShape {
                offsets: [
                    [ivec2(0, 1), ivec2(1, 1), ivec2(2, 1), ivec2(2, 2)],
                    [ivec2(1, 0), ivec2(2, 0), ivec2(1, 1), ivec2(1, 2)],
                    [ivec2(0, 0), ivec2(0, 1), ivec2(1, 1), ivec2(2, 1)],
                    [ivec2(1, 0), ivec2(1, 1), ivec2(0, 2), ivec2(1, 2)],
                ],
                color: Color::srgb_u8(239, 121, 33), // Orange
            },
        }
    }
}

impl TryFrom<u8> for Tetromino {
    type Error = ();

    fn try_from(value: u8) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            x if x == Tetromino::I as u8 => Ok(Tetromino::I),
            x if x == Tetromino::O as u8 => Ok(Tetromino::O),
            x if x == Tetromino::T as u8 => Ok(Tetromino::T),
            x if x == Tetromino::S as u8 => Ok(Tetromino::S),
            x if x == Tetromino::Z as u8 => Ok(Tetromino::Z),
            x if x == Tetromino::J as u8 => Ok(Tetromino::J),
            x if x == Tetromino::L as u8 => Ok(Tetromino::L),
            _ => Err(()),
        }
    }
}
