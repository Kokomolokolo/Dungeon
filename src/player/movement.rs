use avian3d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

use crate::player::Player;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut LinearVelocity, With<Player>>,
) {
    const SPEED: f32 = 2.0;

    for mut velocity in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) {
            direction.z -= 1.0; // In Bevy ist -Z nach vorne
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.z += 1.0; // +Z ist nach hinten
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        velocity.x = direction.x * SPEED;
        velocity.z = direction.z * SPEED;
    }
}