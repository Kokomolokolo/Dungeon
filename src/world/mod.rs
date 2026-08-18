use bevy::prelude::*;

pub mod parser;
pub mod spawner;

use spawner::spawn_map;

use crate::AppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_map);
    }
}