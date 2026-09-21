use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpawnSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PieceMovementSet;
