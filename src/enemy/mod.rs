use bevy::prelude::*;

mod ai;

use ai::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnemyState {
    Idle,
    Chasing,
    Attacking,
}  

#[derive(Component)]
pub struct EnemyStats {
    pub detection_radius: f32,
    pub attack_radius: f32,
    pub speed: f32,
}

impl Default for EnemyStats {
    fn default() -> Self {
        Self {
            detection_radius: 5.0,
            attack_radius: 0.5,
            speed: 1.3
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (enemy_ai, rotate_enemy));
    }
}