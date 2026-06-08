use bevy::{camera::ScalingMode, prelude::*};

use crate::features::board::BoardPlugin;
use crate::features::debug::DebugPlugin;
use crate::features::input::InputPlugin;
use crate::features::piece::PiecePlugin;
use crate::global::components::Position;
use crate::global::util;

mod features;
mod global;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PiecePlugin,
            BoardPlugin,
            InputPlugin,
            DebugPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, sync_position_to_transform)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 20.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn sync_position_to_transform(mut query: Query<(&Position, &mut Transform)>) {
    for (Position(pos), mut transform) in query.iter_mut() {
        transform.translation = util::translate_position_to_grid(*pos);
    }
}
