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
        sets::PieceMovementSet,
        states::{GameState, IsPaused, PlayState},
        util,
    },
};

#[derive(Resource, Deref, DerefMut)]
struct GravityTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct LockTimer(Timer);

#[derive(Resource)]
pub struct ActivePieceState {
    tetromino: Tetromino,
    rotation: usize,
    anchor: Position,
}

impl Default for ActivePieceState {
    fn default() -> Self {
        ActivePieceState {
            tetromino: Tetromino::I,
            rotation: 0,
            anchor: IVec2::ZERO.into(),
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

#[derive(Component)]
struct Clearing {
    timer: Timer,
}

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<PieceLocked>()
            .insert_resource(GravityTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .insert_resource(LockTimer(Timer::from_seconds(0.5, TimerMode::Once)))
            .add_message::<MovePiece>()
            .add_message::<RotatePiece>()
            .add_systems(
                Update,
                (
                    (
                        (move_piece, rotate_piece).in_set(PieceMovementSet),
                        lock_active_piece_on_bottom_collision,
                        resolve_state_after_lock,
                    )
                        .chain()
                        .run_if(in_state(PlayState::Falling)),
                    (animate_clearing_row, delete_filled_row)
                        .chain()
                        .run_if(in_state(PlayState::Clearing)),
                    sync_active_piece_positions,
                )
                    .run_if(in_state(IsPaused::Unpaused)),
            )
            .add_systems(OnEnter(PlayState::Spawning), spawn_next_piece)
            .insert_resource(ActivePieceState::default());

        let debug_config = app.world().get_resource::<DebugConfig>();
        let gravity = debug_config.is_none_or(|config| config.gravity);
        // let auto_start = debug_config.is_none_or(|config| config.auto_start);

        if gravity {
            app.add_systems(
                FixedUpdate,
                apply_gravity
                    .run_if(in_state(PlayState::Falling))
                    .run_if(in_state(IsPaused::Unpaused)),
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

pub fn spawn_piece(
    commands: &mut Commands,
    tetromino: Tetromino,
    anchor: IVec2,
    rotation: usize,
    board: &Board,
) -> bool {
    let block_size = Vec2::splat(1.0 - PADDING_SIZE);
    let piece = tetromino.shape();

    let positions = util::shifted(&piece.offsets[rotation], anchor);
    let is_available_to_occupy = board.can_occupy(&positions);

    for pos in positions.iter() {
        commands.spawn((
            Block,
            Transform::from_translation(util::translate_position_to_grid(*pos)),
            Position(*pos),
            Sprite::from_color(piece.color, block_size), // Add some padding for visual separation
            ActivePiece,
        ));
    }

    is_available_to_occupy
}

fn spawn_next_piece(
    mut commands: Commands,
    mut play_state: ResMut<NextState<PlayState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut lock_timer: ResMut<LockTimer>,
    board: Res<Board>,
) {
    let random = rand::random::<u8>() % 7;
    let tetromino = TETROMINOES.get(random as usize).expect("Random should always be moduloed by the length of the Tetromino enum, so never should be an invalid integer").to_owned();
    // TODO: Make them spawn so that it always touches the top? Or above the board?
    let anchor = ivec2(4, 16);
    let rotation = 0;

    if !spawn_piece(&mut commands, tetromino, anchor, rotation, &board) {
        game_state.set(GameState::Ended);
    } else {
        play_state.set(PlayState::Falling);
    }

    commands.insert_resource(ActivePieceState {
        tetromino,
        rotation,
        anchor: anchor.into(),
    });

    lock_timer.reset();
}

fn lock_active_piece_on_bottom_collision(
    mut commands: Commands,
    active_piece_query: Populated<Entity, With<ActivePiece>>,
    active_piece_state: ResMut<ActivePieceState>,
    mut board: ResMut<Board>,
    mut piece_locked_message: MessageWriter<PieceLocked>,
    time: Res<Time>,
    mut lock_timer: ResMut<LockTimer>,
) {
    let piece_positions: Vec<IVec2> = active_piece_state.positions();

    if board.can_occupy(&util::shifted(&piece_positions, IVec2::NEG_Y)) {
        lock_timer.reset();
        return;
    }

    lock_timer.tick(time.delta());

    if lock_timer.is_finished() {
        for (entity, position) in active_piece_query.iter().zip(piece_positions.iter()) {
            board.set(*position, entity);
            commands.entity(entity).remove::<ActivePiece>();
        }

        piece_locked_message.write(PieceLocked);
    }
}

fn apply_gravity(
    time: Res<Time>,
    mut gravity_timer: ResMut<GravityTimer>,
    board: Res<Board>,
    mut active_piece_state: ResMut<ActivePieceState>,
) {
    gravity_timer.tick(time.delta());

    if gravity_timer.just_finished()
        && board.can_occupy(&util::shifted(
            &active_piece_state.positions(),
            IVec2::NEG_Y,
        ))
    {
        active_piece_state.anchor.shift(IVec2::NEG_Y);
    }
}

fn move_piece(
    mut active_piece_state: ResMut<ActivePieceState>,
    mut reader: MessageReader<MovePiece>,
    mut lock_timer: ResMut<LockTimer>,
    board: Res<Board>,
) {
    for movement in reader.read() {
        let piece_positions = active_piece_state.positions();

        match movement.0 {
            Movement::Down => {
                if board.can_occupy(&util::shifted(&piece_positions, IVec2::NEG_Y)) {
                    active_piece_state.anchor.shift(IVec2::NEG_Y);
                } else {
                    lock_timer.finish();
                }
            }
            Movement::Right => {
                if board.can_occupy(&util::shifted(&piece_positions, IVec2::X)) {
                    lock_timer.reset();
                    active_piece_state.anchor.shift(IVec2::X);
                }
            }
            Movement::Left => {
                if board.can_occupy(&util::shifted(&piece_positions, IVec2::NEG_X)) {
                    lock_timer.reset();
                    active_piece_state.anchor.shift(IVec2::NEG_X);
                }
            }
            Movement::HardDrop => {
                let delta_y = board.get_bottom_legal_position(&piece_positions);

                active_piece_state.anchor.shift(ivec2(0, -delta_y));
                lock_timer.finish();
            }
        }
    }
}

fn rotate_piece(
    mut active_piece_state: ResMut<ActivePieceState>,
    board: Res<Board>,
    mut reader: MessageReader<RotatePiece>,
    mut lock_timer: ResMut<LockTimer>,
) {
    for _ in reader.read() {
        let next_rotation_index = (active_piece_state.rotation + 1) % ROTATION_CYCLES;

        let new_positions = util::shifted(
            &active_piece_state.tetromino.shape().offsets[next_rotation_index],
            *active_piece_state.anchor,
        );

        if board.can_occupy(&new_positions) {
            active_piece_state.rotation = next_rotation_index;
            lock_timer.reset();
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

fn delete_filled_row(
    mut commands: Commands,
    clearing_query: Populated<(Entity, &Clearing, &Position)>,
    mut not_clearing_query: Query<(Entity, &mut Position), (With<Block>, Without<Clearing>)>,
    mut board: ResMut<Board>,
    mut play_state: ResMut<NextState<PlayState>>,
) {
    let mut row_indexes: Vec<i32> = Vec::with_capacity(4);

    for (entity, clearing, position) in &clearing_query {
        if clearing.timer.is_finished() {
            commands.entity(entity).despawn();
            board.remove(position.0);

            if !row_indexes.contains(&position.y) {
                row_indexes.push(position.y);
            }
        }
    }

    not_clearing_query.iter_mut().for_each(|(_, mut position)| {
        let shift_count = row_indexes
            .iter()
            .filter(|row_index| **row_index < position.y)
            .count() as i32;

        if shift_count > 0 {
            board.remove(position.0);
            position.y -= shift_count;
        }
    });

    not_clearing_query.iter().for_each(|(entity, position)| {
        board.set(position.0, entity);
    });

    if clearing_query
        .iter()
        .all(|(_, clearing, _)| clearing.timer.is_finished())
    {
        play_state.set(PlayState::Spawning);
    }
}

fn resolve_state_after_lock(
    mut commands: Commands,
    board: Res<Board>,
    mut reader: MessageReader<PieceLocked>,
    mut play_state: ResMut<NextState<PlayState>>,
) {
    for _ in reader.read() {
        let mut row_cleared = false;

        for row_index in 0..BOARD_HEIGHT {
            let mut block_entities: Vec<Entity> = Vec::with_capacity(BOARD_WIDTH);

            for column_index in 0..BOARD_WIDTH {
                let position = ivec2(column_index as i32, row_index as i32);
                if let Some(entity) = board.get(position) {
                    block_entities.push(entity);
                }
            }

            if block_entities.len() == BOARD_WIDTH {
                row_cleared = true;
                for entity in block_entities {
                    commands.entity(entity).insert(Clearing {
                        timer: Timer::from_seconds(0.3, TimerMode::Once),
                    });
                }
            }
        }

        if row_cleared {
            play_state.set(PlayState::Clearing);
        } else {
            play_state.set(PlayState::Spawning);
        }
    }
}
