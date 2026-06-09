#![allow(unused)] // Debug Mode isn't always used, it's an opt-in module.

use bevy::prelude::*;

use crate::{
    features::{
        board::{self, Board},
        piece,
        tetromino::{TETROMINOES, Tetromino},
    },
    global::{components::Block, resources::DebugConfig, util},
};

#[derive(Resource)]
struct RotationMapIndex(usize);

enum Direction {
    Next,
    Previous,
}

#[derive(Message)]
struct CycleRotationMap(Direction);

#[derive(PartialEq)]
pub enum DebugMode {
    DisableGravity,
    RotationMap,
}

pub struct DebugPlugin(pub DebugMode);

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        match self.0 {
            DebugMode::RotationMap => app
                .add_systems(Startup, init_rotation_map)
                .add_systems(
                    Update,
                    (cycle_rotation_map_on_input, spawn_next_rotation_map).chain(),
                )
                .add_message::<CycleRotationMap>()
                .insert_resource(RotationMapIndex(0))
                .insert_resource(DebugConfig {
                    auto_start: false,
                    gravity: false,
                }),
            DebugMode::DisableGravity => app.insert_resource(DebugConfig {
                auto_start: true,
                gravity: false,
            }),
        };
    }
}

fn spawn_rotation_map(mut commands: Commands, tetromino: Tetromino, board: &Board) {
    for rotation in 0..=3 {
        let anchor = ivec2(rotation * 4, 10);

        commands.spawn((
            Sprite::from_color(Color::srgb(0.3, 0.3, 0.3), Vec2::splat(3.95)),
            Transform::from_translation(
                util::translate_position_to_grid(anchor).with_z(-0.1) + Vec3::new(1.5, 1.5, 0.0),
            ),
        ));

        piece::spawn_piece(&mut commands, tetromino, anchor, rotation as usize, board);
    }
}

fn init_rotation_map(commands: Commands, board: Res<Board>) {
    spawn_rotation_map(commands, TETROMINOES[0], &board);
}

fn spawn_next_rotation_map(
    mut commands: Commands,
    mut index: ResMut<RotationMapIndex>,
    current_tetrominoes: Query<Entity, With<Block>>,
    mut messages: MessageReader<CycleRotationMap>,
    board: Res<Board>,
) {
    for message in messages.read() {
        for entity in current_tetrominoes {
            commands.entity(entity).despawn();
        }

        match message.0 {
            Direction::Next => {
                if index.0 == TETROMINOES.len() - 1 {
                    index.0 = 0;
                } else {
                    index.0 += 1;
                }
            }
            Direction::Previous => {
                if index.0 == 0 {
                    index.0 = TETROMINOES.len() - 1;
                } else {
                    index.0 -= 1;
                }
            }
        }

        spawn_rotation_map(commands.reborrow(), TETROMINOES[index.0], &board);
    }
}

fn cycle_rotation_map_on_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut writer: MessageWriter<CycleRotationMap>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        writer.write(CycleRotationMap(Direction::Previous));
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        writer.write(CycleRotationMap(Direction::Next));
    }
}
