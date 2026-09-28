use bevy::prelude::*;

use crate::global::{constants::*, util};

#[derive(Resource)]
pub struct Board {
    cells: [[Option<Entity>; BOARD_HEIGHT]; BOARD_WIDTH],
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>();
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[None; BOARD_HEIGHT]; BOARD_WIDTH],
        }
    }
}

impl Board {
    pub fn in_bounds(&self, pos: &IVec2) -> bool {
        pos.x >= 0 && pos.x < BOARD_WIDTH as i32 && pos.y >= 0 && pos.y < BOARD_HEIGHT as i32
    }

    pub fn is_occupied(&self, pos: &IVec2) -> bool {
        self.in_bounds(pos) && self.get(*pos).is_some()
    }

    pub fn set(&mut self, pos: IVec2, entity: Entity) {
        if !self.in_bounds(&pos) {
            return;
        }

        self.cells[pos.x as usize][pos.y as usize] = Some(entity);
    }

    pub fn remove(&mut self, pos: IVec2) {
        if !self.in_bounds(&pos) {
            return;
        }

        self.cells[pos.x as usize][pos.y as usize] = None;
    }

    pub fn get(&self, pos: IVec2) -> Option<Entity> {
        if !self.in_bounds(&pos) {
            return None;
        }

        self.cells[pos.x as usize][pos.y as usize]
    }

    pub fn can_occupy(&self, proposed: &[IVec2]) -> bool {
        proposed.iter().all(|proposed_position| {
            self.in_bounds(proposed_position) && !self.is_occupied(proposed_position)
        })
    }

    pub fn get_bottom_legal_position(&self, current: &[IVec2]) -> i32 {
        let mut delta = 0;

        loop {
            let next = util::shifted(current, ivec2(0, -(delta + 1)));

            if !self.can_occupy(&next) {
                return delta;
            }

            delta += 1;
        }
    }
}
