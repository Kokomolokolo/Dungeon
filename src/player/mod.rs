use bevy::prelude::*;

mod movement;

use movement::*;

use crate::AppState;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement.run_if(in_state(AppState::InGame)));
    }
}