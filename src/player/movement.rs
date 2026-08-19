use avian3d::dynamics::rigid_body::LinearVelocity;
use bevy::{gizmos::transform_gizmo, prelude::*};

use crate::player::Player;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &mut Transform), With<Player>>,
    time: Res<Time>
) {
    const SPEED: f32 = 2.0;

    for (mut velocity, mut transform) in &mut query {
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
            // Blickrichtung des Spielers in die Bewegungsrichtug
            let target_angle = direction.x.atan2(direction.z);
            let target_rotation = Quat::from_rotation_y(target_angle);
            transform.rotation = transform.rotation.slerp(target_rotation, 10.0 * time.delta_secs());
            
            direction = direction.normalize();
        }

        velocity.x = direction.x * SPEED;
        velocity.z = direction.z * SPEED;
    }
}