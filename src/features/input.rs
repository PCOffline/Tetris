use bevy::prelude::*;

use crate::features::{board::Board, piece};
use crate::global::{
    components::{ActivePiece, Position},
    util,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_piece_on_keyboard_input);
    }
}

fn move_piece_on_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut active_piece_query: Populated<&mut Position, With<ActivePiece>>,
    board: Res<Board>,
) {
    let piece_positions: Vec<IVec2> = active_piece_query
        .iter()
        .map(|Position(piece)| *piece)
        .collect();

    if (keyboard_input.just_pressed(KeyCode::KeyA)
        || keyboard_input.just_pressed(KeyCode::ArrowLeft))
        && piece::can_occupy(&util::shifted(&piece_positions, ivec2(-1, 0)), &board)
    {
        for mut position in &mut active_piece_query {
            position.0.x -= 1;
        }
    } else if (keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight))
        && piece::can_occupy(&util::shifted(&piece_positions, ivec2(1, 0)), &board)
    {
        for mut position in &mut active_piece_query {
            position.0.x += 1;
        }
    }

    if (keyboard_input.just_pressed(KeyCode::KeyS)
        || keyboard_input.just_pressed(KeyCode::ArrowDown))
        && piece::can_occupy(&util::shifted(&piece_positions, ivec2(0, -1)), &board)
    {
        for mut position in &mut active_piece_query {
            position.0.y -= 1;
        }
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        let delta_y = piece::get_bottom_legal_position(&piece_positions, &board);

        for mut position in &mut active_piece_query.into_iter() {
            position.0.y -= delta_y;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
        // let rotation = collisions::rotate(piece_positions, pivot);

        // if collisions::can_occupy(rotation, &board) {
        // } else {
        // TODO: move pivot point if possible
        // }
    }
}
