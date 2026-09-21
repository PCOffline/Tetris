use bevy::prelude::*;

use crate::global::{
    messages::{MovePiece, Movement, RotatePiece},
    states::{GameState, IsPaused},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_piece_on_keyboard_input.run_if(in_state(IsPaused::Unpaused)),
                toggle_pause_on_keyboard_input.run_if(not(in_state(GameState::Ended))),
            ),
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

fn toggle_pause_on_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    paused_state: Res<State<IsPaused>>,
    mut next_paused_state: ResMut<NextState<IsPaused>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match **paused_state {
            IsPaused::Paused => next_paused_state.set(IsPaused::Unpaused),
            IsPaused::Unpaused => next_paused_state.set(IsPaused::Paused),
        };
    }
}
