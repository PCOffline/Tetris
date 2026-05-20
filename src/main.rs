use bevy::{camera::ScalingMode, prelude::*};

use crate::{
    board::Board,
    piece::{TETROMINOES, Tetromino},
};

mod board;
mod input;
mod piece;
mod util;

const PADDING_SIZE: f32 = 0.05;
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<PieceLocked>()
        .add_message::<GameStarted>()
        .insert_resource(GravityTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .init_resource::<Board>()
        .add_systems(
            Startup,
            (setup_camera, spawn_debug_blocks, spawn_initial_piece),
        )
        .add_systems(FixedUpdate, piece::apply_gravity)
        .add_systems(
            Update,
            (
                piece::lock_active_piece_on_bottom_collision,
                input::move_piece_on_keyboard_input,
                spawn_next_piece,
                sync_position_to_transform,
            )
                .chain(),
        )
        .run();
}

#[derive(Message)]
struct PieceLocked;

#[derive(Message)]
struct GameStarted;

#[derive(Component)]
struct ActivePiece;

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Position(IVec2);

#[derive(Resource)]
struct GravityTimer(Timer);

#[derive(Resource)]
struct ActivePieceState {
    tetromino: Tetromino,
    rotation: u8,
    anchor: IVec2,
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

fn spawn_piece(commands: &mut Commands, tetromino: Tetromino, anchor: IVec2) {
    let block_size = Vec2::splat(1.0 - PADDING_SIZE);
    let piece = tetromino.shape();

    let positions = util::shifted(&piece.offsets, anchor);

    for pos in positions.iter() {
        commands.spawn((
            Block,
            Position(*pos),
            Sprite::from_color(piece.color, block_size), // Add some padding for visual separation
            ActivePiece,
        ));
    }
}

fn spawn_debug_blocks(mut commands: Commands, mut board: ResMut<Board>) {
    let block_size = Vec2::splat(1.0 - PADDING_SIZE);

    let mut spawn_block = |x: i32, y: i32| {
        board.set(
            ivec2(x, y),
            commands
                .spawn((
                    Block,
                    Position(ivec2(x, y)),
                    Sprite::from_color(Color::srgb_u8(0, 255, 0), block_size),
                ))
                .id(),
        )
    };

    spawn_block(1, 2);
    spawn_block(3, 4);
    spawn_block(4, 5);
}

fn spawn_next_piece(
    mut commands: Commands,
    mut piece_locked_reader: MessageReader<PieceLocked>,
    mut game_started_reader: MessageReader<GameStarted>,
) {
    for _ in piece_locked_reader
        .read()
        .map(|_| 0)
        .chain(game_started_reader.read().map(|_| 0))
    {
        let random = rand::random::<u8>() % 7;
        let tetromino = TETROMINOES.get(random as usize).expect("Random should always be moduloed by the length of the Tetromino enum, so never should be an invalid integer").to_owned();

        spawn_piece(&mut commands, tetromino, ivec2(4, 18));
        commands.insert_resource(ActivePieceState {
            tetromino,
            rotation: 0,
            anchor: ivec2(0, 0),
        });
    }
}

fn spawn_initial_piece(mut writer: MessageWriter<GameStarted>) {
    writer.write(GameStarted);
}
