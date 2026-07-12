use bevy::prelude::*;

use crate::{
    features::{
        board::Board,
        piece::{ActivePieceState, PieceLocked, PieceSpawned, get_bottom_legal_position},
    },
    global::{
        components::{ActivePiece, Position},
        constants::PADDING_SIZE,
        sets::SpawnSet,
        states::GameState,
        util,
    },
};

pub struct GhostPlugin;

impl Plugin for GhostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_ghost_pieces, update_ghost).run_if(in_state(GameState::Started)),
        );

        app.add_systems(Update, spawn_ghost_pieces.after(SpawnSet));
    }
}

#[derive(Component)]
pub struct GhostBlock;

fn get_ghost_positions(active_piece_positions: &[IVec2], board: &Board) -> Vec<IVec2> {
    let delta_y = get_bottom_legal_position(active_piece_positions, board);
    util::shifted(active_piece_positions, ivec2(0, -delta_y))
}

fn update_ghost(
    active_piece_state: Res<ActivePieceState>,
    mut ghosts: Query<&mut Position, (With<GhostBlock>, Without<ActivePiece>)>,
    board: Res<Board>,
) {
    if !active_piece_state.is_changed() && !board.is_changed() {
        return;
    }

    let active_piece_positions = active_piece_state.positions();
    let ghost_positions = get_ghost_positions(&active_piece_positions, &board);

    for (mut position, target) in ghosts.iter_mut().zip(ghost_positions.iter()) {
        *position = Position(*target);
    }
}

fn spawn_ghost_pieces(
    mut commands: Commands,
    active_piece_state: Res<ActivePieceState>,
    board: Res<Board>,
    mut piece_spawned_reader: MessageReader<PieceSpawned>,
) {
    for _ in piece_spawned_reader.read() {
        let block_size = Vec2::splat(1.0 - PADDING_SIZE);
        let ghost_positions = get_ghost_positions(&active_piece_state.positions(), &board);

        for pos in ghost_positions.iter() {
            commands.spawn((
                GhostBlock,
                Transform::from_translation(util::translate_position_to_grid(*pos)),
                Position(*pos),
                Sprite::from_color(
                    active_piece_state.shape().color.with_alpha(0.15),
                    block_size,
                ),
            ));
        }
    }
}

fn despawn_ghost_pieces(
    mut piece_locked_reader: MessageReader<PieceLocked>,
    ghost_pieces: Query<Entity, With<GhostBlock>>,
    mut commands: Commands,
) {
    for _ in piece_locked_reader.read() {
        for entity in ghost_pieces.iter() {
            commands.entity(entity).despawn();
        }
    }
}
