use bevy::{log::tracing::span::Id, prelude::*};

mod movement;
mod animations;

use movement::*;

use crate::AppState;

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::Walking,
        }
    }
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub graph: Handle<AnimationGraph>,
    pub idle: AnimationNodeIndex,
    pub walking: AnimationNodeIndex,
}

pub enum PlayerState {
    Idle, 
    Walking, 
    Attack,
    Die
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_movement.run_if(in_state(AppState::InGame)));
    }
}