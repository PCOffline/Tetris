use bevy::prelude::*;

use crate::{
    features::{
        board::Board,
        tetromino::{PieceShape, TETROMINOES, Tetromino},
    },
    global::{
        components::{ActivePiece, Block, Position},
        constants::*,
        messages::{MovePiece, Movement, RotatePiece},
        resources::DebugConfig,
        sets::SpawnSet,
        states::GameState,
        util,
    },
};

#[derive(Resource)]
struct GravityTimer(Timer);

#[derive(Resource)]
pub struct ActivePieceState {
    tetromino: Tetromino,
    rotation: usize,
    anchor: Position,
    lock_timer: Timer,
}

impl Default for ActivePieceState {
    fn default() -> Self {
        ActivePieceState {
            tetromino: Tetromino::I,
            rotation: 0,
            anchor: IVec2::ZERO.into(),
            lock_timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

impl ActivePieceState {
    pub fn shape(&self) -> PieceShape {
        self.tetromino.shape()
    }

    pub fn positions(&self) -> Vec<IVec2> {
        util::shifted(&self.tetromino.shape().offsets[self.rotation], *self.anchor)
    }
}

#[derive(Message)]
pub struct PieceLocked;

#[derive(Message)]
struct GameStarted;

#[derive(Message)]
pub struct PieceSpawned;

#[derive(Component)]
struct Clearing {
    timer: Timer,
}

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PieceLocked>()
            .insert_resource(GravityTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_message::<GameStarted>()
            .add_message::<MovePiece>()
            .add_message::<RotatePiece>()
            .add_message::<PieceSpawned>()
            .add_systems(
                Update,
                (
                    move_piece,
                    rotate_piece,
                    sync_active_piece_positions,
                    lock_active_piece_on_bottom_collision,
                    clear_filled_row,
                    animate_clearing_row,
                    delete_filled_row,
                )
                    .run_if(in_state(GameState::Started))
                    .chain(),
            )
            .insert_resource(ActivePieceState::default());

        let debug_config = app.world().get_resource::<DebugConfig>();
        let gravity = debug_config.is_none_or(|config| config.gravity);
        let auto_start = debug_config.is_none_or(|config| config.auto_start);

        if auto_start {
            app.add_systems(Startup, spawn_initial_piece).add_systems(
                Update,
                spawn_next_piece
                    .in_set(SpawnSet)
                    .run_if(in_state(GameState::Started)),
            );
        }

        if gravity {
            app.add_systems(
                FixedUpdate,
                apply_gravity.run_if(in_state(GameState::Started)),
            );
        }
    }
}

fn sync_active_piece_positions(
    active_piece_state: Res<ActivePieceState>,
    mut query: Query<&mut Position, With<ActivePiece>>,
) {
    if !active_piece_state.is_changed() {
        return;
    }

    let piece_positions = active_piece_state.positions();

    for (mut position, target) in query.iter_mut().zip(piece_positions.iter()) {
        **position = *target;
    }
}

fn can_occupy(proposed: &[IVec2], board: &Board) -> bool {
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

pub fn spawn_piece(
    commands: &mut Commands,
    tetromino: Tetromino,
    anchor: IVec2,
    rotation: usize,
    board: &Board,
) -> bool {
    let block_size = Vec2::splat(1.0 - PADDING_SIZE);
    let piece = tetromino.shape();

    let mut positions = util::shifted(&piece.offsets[rotation], anchor);
    let occupied = can_occupy(&positions, board);

    if occupied {
        positions.retain(|pos| !board.is_occupied(pos));
    }

    for pos in positions.iter() {
        commands.spawn((
            Block,
            Transform::from_translation(util::translate_position_to_grid(*pos)),
            Position(*pos),
            Sprite::from_color(piece.color, block_size), // Add some padding for visual separation
            ActivePiece,
        ));
    }

    occupied
}

fn spawn_initial_piece(mut writer: MessageWriter<GameStarted>) {
    writer.write(GameStarted);
}

fn spawn_next_piece(
    mut commands: Commands,
    mut piece_locked_reader: MessageReader<PieceLocked>,
    mut game_started_reader: MessageReader<GameStarted>,
    mut piece_spawned_writer: MessageWriter<PieceSpawned>,
    board: Res<Board>,
) {
    for _ in piece_locked_reader
        .read()
        .map(|_| ())
        .chain(game_started_reader.read().map(|_| ()))
    {
        let random = rand::random::<u8>() % 7;
        let tetromino = TETROMINOES.get(random as usize).expect("Random should always be moduloed by the length of the Tetromino enum, so never should be an invalid integer").to_owned();
        // TODO: Make them spawn so that it always touches the top? Or above the board?
        let anchor = ivec2(4, 16);
        let rotation = 0;

        if !spawn_piece(&mut commands, tetromino, anchor, rotation, &board) {
            commands.set_state(GameState::Ended);
        }

        commands.insert_resource(ActivePieceState {
            tetromino,
            rotation,
            anchor: anchor.into(),
            lock_timer: Timer::from_seconds(0.5, TimerMode::Once),
        });

        piece_spawned_writer.write(PieceSpawned);
    }
}

fn lock_active_piece_on_bottom_collision(
    mut commands: Commands,
    active_piece_query: Populated<(Entity, &Position), With<ActivePiece>>,
    mut active_piece_state: ResMut<ActivePieceState>,
    mut board: ResMut<Board>,
    mut piece_locked_message: MessageWriter<PieceLocked>,
    time: Res<Time>,
) {
    active_piece_state.lock_timer.tick(time.delta());

    let positions: Vec<IVec2> = active_piece_query.iter().map(|(_, pos)| **pos).collect();

    if active_piece_state.lock_timer.is_finished()
        && !can_occupy(&util::shifted(&positions, IVec2::NEG_Y), &board)
    {
        for (entity, position) in &active_piece_query {
            board.set(**position, entity);
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
    mut active_piece_state: ResMut<ActivePieceState>,
) {
    gravity_timer.0.tick(time.delta());

    let positions: Vec<IVec2> = query.iter().map(|position| **position).collect();
    if gravity_timer.0.just_finished()
        && can_occupy(&util::shifted(&positions, IVec2::NEG_Y), &board)
    {
        for mut position in query.iter_mut() {
            position.shift(IVec2::NEG_Y);
        }

        active_piece_state.anchor.shift(IVec2::NEG_Y);
        active_piece_state.lock_timer.reset();
    }
}

fn move_piece(
    active_piece_query: Populated<&mut Position, With<ActivePiece>>,
    mut active_piece_state: ResMut<ActivePieceState>,
    mut reader: MessageReader<MovePiece>,
    board: Res<Board>,
) {
    let piece_positions: Vec<IVec2> = active_piece_query
        .iter()
        .map(|position| **position)
        .collect();

    for movement in reader.read() {
        match movement.0 {
            Movement::Down => {
                if can_occupy(&util::shifted(&piece_positions, IVec2::NEG_Y), &board) {
                    active_piece_state.anchor.shift(IVec2::NEG_Y);
                } else {
                    active_piece_state.lock_timer.finish();
                }
            }
            Movement::Right => {
                if can_occupy(&util::shifted(&piece_positions, IVec2::X), &board) {
                    active_piece_state.lock_timer.reset();
                    active_piece_state.anchor.shift(IVec2::X);
                }
            }
            Movement::Left => {
                if can_occupy(&util::shifted(&piece_positions, IVec2::NEG_X), &board) {
                    active_piece_state.lock_timer.reset();
                    active_piece_state.anchor.shift(IVec2::NEG_X);
                }
            }
            Movement::HardDrop => {
                let delta_y = get_bottom_legal_position(&piece_positions, &board);

                active_piece_state.anchor.shift(ivec2(0, -delta_y));
                active_piece_state.lock_timer.finish();
            }
        }
    }
}

fn rotate_piece(
    mut active_piece_state: ResMut<ActivePieceState>,
    board: Res<Board>,
    mut reader: MessageReader<RotatePiece>,
) {
    for _ in reader.read() {
        let next_rotation_index = (active_piece_state.rotation + 1) % ROTATION_CYCLES;

        let new_positions = util::shifted(
            &active_piece_state.tetromino.shape().offsets[next_rotation_index],
            *active_piece_state.anchor,
        );

        if can_occupy(&new_positions, &board) {
            active_piece_state.rotation = next_rotation_index;
        }
    }
}

fn animate_clearing_row(query: Populated<(&mut Sprite, &mut Clearing)>, time: Res<Time>) {
    for (mut sprite, mut clearing) in query {
        clearing.timer.tick(time.delta());

        if !clearing.timer.is_finished() {
            sprite.color.set_alpha(1.0 - clearing.timer.fraction());
        }
    }
}

fn delete_filled_row(mut commands: Commands, query: Populated<(Entity, &Clearing)>) {
    for (entity, clearing) in query {
        if clearing.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn clear_filled_row(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut reader: MessageReader<PieceLocked>,
) {
    for _ in reader.read() {
        for row_index in 0..BOARD_HEIGHT {
            let mut block_entities: Vec<(Entity, IVec2)> = Vec::with_capacity(BOARD_WIDTH);

            for column_index in 0..BOARD_WIDTH {
                let position = ivec2(column_index as i32, row_index as i32);
                if let Some(entity) = board.get(position) {
                    block_entities.push((entity, position));
                }
            }

            if block_entities.len() == BOARD_WIDTH {
                for (entity, position) in block_entities {
                    commands.entity(entity).insert(Clearing {
                        timer: Timer::from_seconds(0.3, TimerMode::Once),
                    });
                    board.remove(position);
                }
            }
        }
    }
}
