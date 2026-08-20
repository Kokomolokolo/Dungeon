use bevy::prelude::*;

mod movement;
mod animations;
mod state;
mod weapon;


use movement::*;
use animations::*;
use state::*;
use weapon::*;

use crate::AppState;

#[derive(Component, Debug)]
pub struct Player {
    pub state: PlayerState,
}
impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::Idle,
        }
    }
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub graph: Handle<AnimationGraph>,
    pub idle: AnimationNodeIndex,
    pub walking: AnimationNodeIndex,
    pub attack: AnimationNodeIndex,
    pub die: AnimationNodeIndex,
    pub pose: AnimationNodeIndex, // eigentlich static, aber darf wegen name konflikten nicht so gennant werden
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum PlayerState {
    Idle, 
    Walking, 
    Attack,
    Die
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, debug_print_names);
        app.add_systems(Update, attach_weapon_player);
        app.add_systems(Update, (player_attack, player_movement, update_player_state).chain().run_if(in_state(AppState::InGame)));
        app.add_plugins(PlayerAnimationPlugin);
    }
}

fn debug_print_names(query: Query<&Name, Added<Name>>) {
    for name in &query {
        println!("Gefundenes Objekt/Knochen im Modell: {}", name.as_str());
       

    }
    println!();
}