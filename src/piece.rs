use bevy::prelude::*;

use crate::{ActivePiece, GravityTimer, PieceLocked, Position, board::Board, util};

pub struct PieceShape {
    pub offsets: [IVec2; 4],
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
                offsets: [ivec2(0, 0), ivec2(1, 0), ivec2(2, 0), ivec2(3, 0)],
                color: Color::srgb_u8(49, 199, 239), // Light blue
            },
            Self::O => PieceShape {
                offsets: [ivec2(0, 0), ivec2(0, 1), ivec2(1, 0), ivec2(1, 1)],
                color: Color::srgb_u8(247, 211, 8), // Yellow
            },
            Self::T => PieceShape {
                offsets: [ivec2(1, 0), ivec2(0, 1), ivec2(1, 1), ivec2(2, 0)],
                color: Color::srgb_u8(179, 77, 156), // Purple
            },
            Self::S => PieceShape {
                offsets: [ivec2(1, 0), ivec2(2, 0), ivec2(0, 1), ivec2(1, 1)],
                color: Color::srgb_u8(66, 182, 66), // Green
            },
            Self::Z => PieceShape {
                offsets: [ivec2(0, 0), ivec2(1, 0), ivec2(1, 1), ivec2(2, 1)],
                color: Color::srgb_u8(239, 32, 41), // Red
            },
            Self::J => PieceShape {
                offsets: [ivec2(0, 0), ivec2(0, 1), ivec2(1, 1), ivec2(2, 1)],
                color: Color::srgb_u8(90, 101, 173), // Blue
            },
            Self::L => PieceShape {
                offsets: [ivec2(2, 0), ivec2(0, 1), ivec2(1, 1), ivec2(2, 1)],
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

pub fn can_occupy(proposed: &[IVec2], board: &Board) -> bool {
    proposed.iter().all(|proposed_position| {
        board.in_bounds(proposed_position) && !board.is_occupied(proposed_position)
    })
}

pub fn apply_gravity(
    mut query: Query<&mut Position, With<ActivePiece>>,
    time: Res<Time>,
    mut gravity_timer: ResMut<GravityTimer>,
    board: Res<Board>,
) {
    gravity_timer.0.tick(time.delta());

    let positions: Vec<IVec2> = query.iter().map(|Position(piece)| *piece).collect();
    if gravity_timer.0.just_finished()
        && can_occupy(&util::shifted(&positions, ivec2(0, -1)), &board)
    {
        for mut position in query.iter_mut() {
            position.0.y -= 1;
        }
    }
}

pub fn lock_active_piece_on_bottom_collision(
    mut commands: Commands,
    active_piece_query: Populated<(Entity, &Position), With<ActivePiece>>,
    mut board: ResMut<Board>,
    mut piece_locked_message: MessageWriter<PieceLocked>,
) {
    let positions: Vec<IVec2> = active_piece_query
        .iter()
        .map(|(_, Position(pos))| *pos)
        .collect();

    if !can_occupy(&util::shifted(&positions, IVec2::new(0, -1)), &board) {
        for (entity, Position(pos)) in &active_piece_query {
            board.set(*pos, entity);
            commands.entity(entity).remove::<ActivePiece>();
        }

        piece_locked_message.write(PieceLocked);
    }
}
pub fn get_bottom_legal_position(current: &[IVec2], board: &Board) -> i32 {
    let mut delta = 0;

    loop {
        let next = util::shifted(current, ivec2(0, -(delta + 1)));

        if !can_occupy(&next, board) {
            return delta;
        }

        delta += 1;
    }
}
