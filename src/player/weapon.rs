use bevy::prelude::*;

use crate::assets::DungeonAssets;

pub fn attach_weapon_player(
    mut commands: Commands,
    dungeon_assets: Res<DungeonAssets>,
    bone_query: Query<(Entity, &Name), Added<Name>> // Bevy gibt jedem Knochen ein "Name", deswegen können knochen so gefunden werden
) {
    for (entity, name) in bone_query {
        if name.as_str() == "arm-right" {
            let sword_handle = dungeon_assets.assets.get("weapon-sword.glb");
            match sword_handle {
                Some(handle) => {
                    let sword = commands.spawn((
                        WorldAssetRoot(handle.clone()),
                        Transform::from_xyz(-0.3, -0.0, 0.1)
                            .with_rotation(Quat::from_euler(
                                EulerRot::XYZ, 
                                0.0_f32.to_radians(), 
                                0.0_f32.to_radians(), 
                                0.0_f32.to_radians()
                            ))
                    )).id();
                    println!("Spawned Entity");

                    commands.entity(entity).add_child(sword);
                }
                None => {
                    eprintln!("Couldnt find sword");
                    panic!()
                }
            }
        }
    }
}