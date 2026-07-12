use bevy::{camera::ScalingMode, prelude::*};

use crate::features::board::BoardPlugin;
use crate::features::debug::{DebugMode, DebugPlugin};
use crate::features::ghost::GhostPlugin;
use crate::features::input::InputPlugin;
use crate::features::piece::PiecePlugin;
use crate::global::components::Position;
use crate::global::constants::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::global::states::GameState;
use crate::global::util;

mod features;
mod global;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // DebugPlugin(DebugMode::DisableGravity),
            PiecePlugin,
            BoardPlugin,
            InputPlugin,
            GhostPlugin,
        ))
        .add_systems(Startup, (setup_camera, draw_borders))
        .add_systems(Update, sync_position_to_transform)
        .insert_state(GameState::Started)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: BOARD_HEIGHT as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn draw_borders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(BOARD_WIDTH as f32, BOARD_HEIGHT as f32));
    let color = Color::srgb_u8(50, 50, 50);

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(color)),
        Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
    ));
}

fn sync_position_to_transform(mut query: Query<(&Position, &mut Transform)>) {
    for (Position(pos), mut transform) in query.iter_mut() {
        transform.translation = util::translate_position_to_grid(*pos);
    }
}
