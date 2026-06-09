use bevy::prelude::*;

use crate::global::{
    messages::{MovePiece, Movement, RotatePiece},
    states::GameState,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_piece_on_keyboard_input.run_if(in_state(GameState::Started)),
        );
    }
}

fn move_piece_on_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut rotate_piece_writer: MessageWriter<RotatePiece>,
    mut move_piece_writer: MessageWriter<MovePiece>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        move_piece_writer.write(MovePiece(Movement::Left));
    } else if keyboard_input.just_pressed(KeyCode::KeyD)
        || keyboard_input.just_pressed(KeyCode::ArrowRight)
    {
        move_piece_writer.write(MovePiece(Movement::Right));
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        move_piece_writer.write(MovePiece(Movement::Down));
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        move_piece_writer.write(MovePiece(Movement::HardDrop));
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
        rotate_piece_writer.write(RotatePiece);
    }
}
