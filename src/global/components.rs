use bevy::prelude::*;

#[derive(Component)]
pub struct ActivePiece;

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct Block;
