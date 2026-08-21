use avian3d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

use crate::{enemy::{Enemy, EnemyState, EnemyStats}, player::Player};


pub fn enemy_ai(
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(&Transform, &EnemyStats, &mut EnemyState, &mut LinearVelocity), With<Enemy>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    for ( enemy_trans, stats_enemy, mut state_enemy, mut velocity_enemy) in enemy_query {
        let distance = enemy_trans.translation.distance(player_transform.translation);

        if distance <= stats_enemy.attack_radius {
            *state_enemy = EnemyState::Attacking;
            velocity_enemy.x = 0.0;
            velocity_enemy.z = 0.0;
        } else  if distance <= stats_enemy.detection_radius {
            *state_enemy = EnemyState::Chasing;

            // In Richtung des Spielers bewegen
            let direction = (player_transform.translation - enemy_trans.translation).normalize();
            velocity_enemy.x = direction.x * stats_enemy.speed;
            velocity_enemy.z = direction.z * stats_enemy.speed;
        } else {
            *state_enemy = EnemyState::Idle;
            velocity_enemy.x = 0.0;
            velocity_enemy.z = 0.0;
        }
    }
}

pub fn rotate_enemy(enemy_query: Query<(&mut Transform, &LinearVelocity), With<Enemy>>, time: Res<Time>) {
    for (mut enemy_trans, velocity) in enemy_query {    
        // Den Gegner in richtige Richtung drehen
        if velocity.length_squared() > 0.1 {
            let target_angle = velocity.x.atan2(velocity.z);
            let target_rotation = Quat::from_rotation_y(target_angle);
            enemy_trans.rotation = enemy_trans.rotation.slerp(target_rotation, 10.0 * time.delta_secs())
        }
    }
}