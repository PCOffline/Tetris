use bevy::prelude::*;

use crate::{
    features::{
        board::Board,
        tetromino::{TETROMINOES, Tetromino},
    },
    global::{
        components::{ActivePiece, Block, Position},
        constants::*,
        resources::DebugMode,
        util,
    },
};

#[derive(Resource)]
struct GravityTimer(Timer);

#[derive(Resource)]
struct ActivePieceState {
    tetromino: Tetromino,
    rotation: u8,
}

#[derive(Message)]
struct PieceLocked;

#[derive(Message)]
struct GameStarted;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PieceLocked>()
            .insert_resource(GravityTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_message::<GameStarted>()
            .add_systems(
                Startup,
                spawn_initial_piece.run_if(not(resource_exists::<DebugMode>)),
            )
            .add_systems(
                Update,
                (spawn_next_piece, lock_active_piece_on_bottom_collision).chain(),
            )
            .add_systems(FixedUpdate, apply_gravity);
    }
}

pub fn can_occupy(proposed: &[IVec2], board: &Board) -> bool {
    proposed.iter().all(|proposed_position| {
        board.in_bounds(proposed_position) && !board.is_occupied(proposed_position)
    })
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

pub fn spawn_piece(commands: &mut Commands, tetromino: Tetromino, anchor: IVec2, rotation: usize) {
    let block_size = Vec2::splat(1.0 - PADDING_SIZE);
    let piece = tetromino.shape();

    let positions = util::shifted(&piece.offsets[rotation], anchor);

    for pos in positions.iter() {
        commands.spawn((
            Block,
            Position(*pos),
            Sprite::from_color(piece.color, block_size), // Add some padding for visual separation
                                                         // ActivePiece,
        ));
    }
}

fn spawn_initial_piece(mut writer: MessageWriter<GameStarted>) {
    writer.write(GameStarted);
}

fn spawn_next_piece(
    mut commands: Commands,
    mut piece_locked_reader: MessageReader<PieceLocked>,
    mut game_started_reader: MessageReader<GameStarted>,
) {
    for _ in piece_locked_reader
        .read()
        .map(|_| ())
        .chain(game_started_reader.read().map(|_| ()))
    {
        let random = rand::random::<u8>() % 7;
        let tetromino = TETROMINOES.get(random as usize).expect("Random should always be moduloed by the length of the Tetromino enum, so never should be an invalid integer").to_owned();

        spawn_piece(&mut commands, tetromino, ivec2(4, 18), 0);
        commands.insert_resource(ActivePieceState {
            tetromino,
            rotation: 0,
        });
    }
}

fn lock_active_piece_on_bottom_collision(
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

fn apply_gravity(
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
