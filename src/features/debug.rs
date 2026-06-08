use bevy::prelude::*;

use crate::{
    features::{piece, tetromino::Tetromino},
    global::{resources::DebugMode, util},
};

#[allow(unused)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_debug_blocks)
            .insert_resource(DebugMode);
    }
}

#[allow(unused)]
fn spawn_debug_blocks(mut commands: Commands) {
    for (index, tetromino) in [Tetromino::O, Tetromino::T, Tetromino::I]
        .into_iter()
        .enumerate()
    {
        for rotation in 0..=3 {
            let anchor = ivec2(rotation * 4, index as i32 * 4);

            commands.spawn((
                Sprite::from_color(Color::srgb(0.3, 0.3, 0.3), Vec2::splat(3.95)),
                Transform::from_translation(
                    util::translate_position_to_grid(anchor).with_z(-0.1)
                        + Vec3::new(1.5, 1.5, 0.0),
                ),
            ));

            piece::spawn_piece(&mut commands, tetromino, anchor, rotation as usize);
        }
    }
}
