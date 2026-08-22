use bevy::prelude::*;
use avian3d::prelude::*;

use crate::player::PlayerPlugin;
use crate::test_world::TestWorldPlugin;
use crate::camera::CameraPlugin;
use crate::assets::AssetPlugin;
use crate::world::WorldPlugin;
use crate::enemy::EnemyPlugin;
use crate::menu::MenuPlugin;
use crate::combat::*;

mod test_world;
mod camera;
mod world;
mod assets;
mod player;
mod enemy;
mod menu;
mod combat;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    InGame,
}


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(PhysicsDebugPlugin::default())
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_plugins((MenuPlugin, WorldPlugin, CameraPlugin, AssetPlugin, PlayerPlugin, TestWorldPlugin, EnemyPlugin, CombatPlugin))
        .run();
}

fn setup(mut commands: Commands) {
    //commands.spawn((
    //    Camera3d::default(),
    //    Transform::from_xyz(0., 0. , 10.).looking_at(Vec3::ZERO, Vec3::Y)
    //));
    commands.spawn((
        AmbientLight {
            brightness: 0.5, 
            affects_lightmapped_meshes: false,
            ..default()
        },
    ));
    commands.insert_resource(ClearColor(Color::BLACK));
}