use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Component)]
pub struct ActivePiece;

#[derive(Component)]
pub struct Position(pub IVec2);

impl Position {
    pub fn shift(&mut self, delta: IVec2) {
        self.0 += delta;
    }
}

impl Deref for Position {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<IVec2> for Position {
    fn from(value: IVec2) -> Self {
        Position(value)
    }
}

#[derive(Component)]
pub struct Block;
