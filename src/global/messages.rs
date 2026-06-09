use bevy::prelude::*;

#[derive(Message)]
pub struct RotatePiece;

pub enum Movement {
    Down,
    Left,
    Right,
    HardDrop,
}

#[derive(Message)]
pub struct MovePiece(pub Movement);
