// Spawnt die Teile der Map
// 
use bevy::prelude::*;

use crate::{assets::DungeonAssets, world::parser::{TileType, parse_map}};

const MAP: &str = "\
##########
#........#
#.P..#...#
#....#...#
##########";

pub fn spawn_map(mut commands: Commands, dungeon_assets: Res<DungeonAssets>) {
    let map_data = parse_map(MAP);
    let tile_size = 2.;
    let scale = 2.0;

    for (pos, tile_type) in map_data.tiles {
        let asset_name = match tile_type {
            TileType::Wall => "wall.glb",
            TileType::Floor => "floor.glb",
            TileType::PlayerSpawn => "floor.glb",
        };

        if let Some(asset) = dungeon_assets.assets.get(asset_name) {
            // Das Asset wurde gefunden
            commands.spawn((
                WorldAssetRoot(asset.clone()),
                Transform::from_xyz(pos.x as f32 * tile_size, 0.0, pos.y as f32 * tile_size).with_scale(Vec3::splat(scale))
            ));
        }
    }
}