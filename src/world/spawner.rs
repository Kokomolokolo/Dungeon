use avian3d::{collision::collider::Collider, dynamics::rigid_body::{LockedAxes, RigidBody}};
// Spawnt die Teile der Map
// 
use bevy::prelude::*;

use crate::{assets::DungeonAssets, player::Player, world::parser::{TileType, parse_map}};

const MAP: &str = "\
##########
#........#
#........#
#........#
#.P..#...#
#....#...#
##########";

pub fn spawn_map(mut commands: Commands, dungeon_assets: Res<DungeonAssets>) {
    let map_data = parse_map(MAP);
    let tile_size = 1.;
    let scale = 1.;

    for (pos, tile_type) in map_data.tiles {
        let asset_name = match tile_type {
            TileType::Wall => "wall.glb",
            TileType::Floor => "floor.glb",
            TileType::PlayerSpawn => "floor.glb",
        };

        let height = if asset_name == "floor.glb" {
            0.1
            } else {
            2.0
            };

        if let Some(asset) = dungeon_assets.assets.get(asset_name) {
            // Das Asset wurde gefunden
            commands.spawn((
                WorldAssetRoot(asset.clone()),
                Transform::from_xyz(pos.x as f32 * tile_size, 0.0, pos.y as f32 * tile_size).with_scale(Vec3::splat(scale)),

                // Für die Kollision
                // static riget body mit einem Collider mit der Größe des Würfels
                RigidBody::Static,
                Collider::cuboid(tile_size, height, tile_size) // breite, höhe, länge
            ));
        }

        // Spawn des Spielers
        // Spieler wird weil es einfacher ist hier gespawnt.
        if tile_type == TileType::PlayerSpawn {
            if let Some(player_handle) = dungeon_assets.assets.get("character-human.glb") {
                commands.spawn((
                    // Die Hitbox als Parent, und das Model als child
                    Player,
                    RigidBody::Dynamic, // Dynamisch, reagiert mit anderen Physik Objekten
                    Collider::capsule(0.3, 0.3), // Runder collider: radius, höhe an der y achse
                    LockedAxes::ROTATION_LOCKED, // Verhindert, das Spieler umkippt
                    Transform::from_xyz(pos.x as f32 * tile_size, 0.1, pos.y as f32 * tile_size).with_scale(Vec3::splat(scale)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        WorldAssetRoot(player_handle.clone()),
                        // Verschieben der Höhe des Models
                        Transform::from_xyz(0.0, -0.4, 0.0)
                            .with_scale(Vec3::splat(scale)),
                    ));
                    
                });
            }
        }
    }
}