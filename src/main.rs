use bevy::prelude::*;

use crate::test_world::TestWorldPlugin;
use crate::camera::CameraPlugin;
use crate::assets::AssetPlugin;
use crate::world::WorldPlugin;


mod test_world;
mod camera;
mod world;
mod assets;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins((WorldPlugin, CameraPlugin, AssetPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    //commands.spawn((
    //    Camera3d::default(),
    //    Transform::from_xyz(0., 0. , 10.).looking_at(Vec3::ZERO, Vec3::Y)
    //));
    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y)
    ));
}