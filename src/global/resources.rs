use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct DebugConfig {
    pub auto_start: bool,
    pub gravity: bool,
}
