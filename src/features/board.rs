use bevy::prelude::*;

use crate::global::constants::*;

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
}
