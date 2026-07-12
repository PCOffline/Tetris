use bevy::math::{IVec2, Vec3};

use crate::global::constants::*;

pub fn shifted(positions: &[IVec2], delta: IVec2) -> Vec<IVec2> {
    positions.iter().map(|position| position + delta).collect()
}

pub fn translate_position_to_grid(pos: IVec2) -> Vec3 {
    Vec3::new(
        pos.x as f32 - BOARD_WIDTH as f32 / 2.0 + 0.5,
        pos.y as f32 - BOARD_HEIGHT as f32 / 2.0 + 0.5,
        0.0,
    )
}
