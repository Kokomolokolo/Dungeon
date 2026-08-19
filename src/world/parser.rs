use std::collections::HashMap;

use bevy::prelude::*;

#[derive(PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    PlayerSpawn, // Wird zu einem Floor, nur um es auf der Map zu notieren
}

pub struct MapData {
    pub tiles: HashMap<IVec2, TileType>
}

pub fn parse_map(aascii_str: &str) -> MapData {
    let mut tiles = HashMap::new();

    // Iteriert über Zeilen, wobei y die Zeilenummer
    for (y, line) in aascii_str.lines().enumerate() {
        // x = Spaltennummer
        for (x, character) in line.chars().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);

            // Match über die tiles
            let tile = match character {
                '#' => Some(TileType::Wall),
                '.' => Some(TileType::Floor),
                'P' => Some(TileType::PlayerSpawn),
                _ => None,
            };

            if let Some(t) = tile {
                tiles.insert(pos, t);
            }
        }
    }
    MapData { tiles }
}