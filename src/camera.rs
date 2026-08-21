use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::AppState;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_camera);
        app.add_systems(Update, (
            camera_look, 
            camera_movement, 
            lock_cursor_on_click,
            unlock_cursor_esc,
            camera_focus_player
        ).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct FpsCamera {
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for FpsCamera {
    fn default() -> Self {
        Self {
            speed: 20.0,
            sensitivity: 0.001,
        }
    }
}

pub fn setup_camera(
    mut commands: Commands,
    // Nutzt "Single" statt "Query", da es nur ein PrimaryWindow gibt
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 12.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        FpsCamera::default(),
    ));
}

pub fn lock_cursor_on_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
    }
}

fn unlock_cursor_esc(
    keys: Res<ButtonInput<KeyCode>>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
    }
}

pub fn camera_movement(
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &FpsCamera)>,
) {
    for (mut transform, fps_cam) in &mut query {
        let mut velocity = Vec3::ZERO;

        let forward = transform.forward();
        let right = transform.right();

        if key.pressed(KeyCode::ArrowUp) {
            velocity += *forward;
        }
        if key.pressed(KeyCode::ArrowDown) {
            velocity -= *forward;
        }
        if key.pressed(KeyCode::ArrowLeft) {
            velocity -= *right;
        }
        if key.pressed(KeyCode::ArrowRight) {
            velocity += *right;
        }

        if key.pressed(KeyCode::Space) {
            velocity.y += 1.0;
        }
        if key.pressed(KeyCode::ShiftLeft) {
            velocity.y -= 1.0;
        }
        if key.pressed(KeyCode::KeyT) {
            println!("{:?}", transform);
        }

        transform.translation += velocity.normalize_or_zero() * fps_cam.speed * time.delta_secs();
    }
}

pub fn camera_focus_player(mut camera_query: Query<&mut Transform, With<FpsCamera>>, player_query: Query<& Transform, (With<Player>, Without<FpsCamera>)>) {
    // Hier immer das looking at der Kamera auf den Spieler machen und ihn verfolgen
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    if let Ok(mut camera_transform) = camera_query.single_mut() {
        camera_transform.look_at(player_transform.translation, Vec3::Y);
    }
}

pub fn camera_look(
    mut mouse_motion: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &FpsCamera)>,
) {
    for (mut transform, fps_cam) in &mut query {
        for motion in mouse_motion.read() {
            let yaw = -motion.delta.x * fps_cam.sensitivity;
            let pitch = -motion.delta.y * fps_cam.sensitivity;
            
            transform.rotate_y(yaw);
            transform.rotate_local_x(pitch);
        }
    }
}