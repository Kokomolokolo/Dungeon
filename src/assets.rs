use std::collections::HashMap;

use bevy::prelude::*;

use crate::test_world::get_model_filenames;
use crate::AppState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), load_models);
    }
}

// 1. Define the resource to hold the GLTF handle
#[derive(Resource, Debug)]
pub struct DungeonAssets {
    // Die assets mit file namen und ihrem handle
    pub assets: HashMap<String, Handle<WorldAsset>>,
}

fn load_models(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut next_state: ResMut<NextState<AppState>>, // State changer
) {
    // Filenames bekommen
    match get_model_filenames("./assets/models") {
        Ok(files) => {
            let mut assets = HashMap::new();
            for file in files {
                // Lennart sag dazu einfach gar nichts
                assets.insert(
                    file.clone(), 
                    asset_server.load(
                        GltfAssetLabel::Scene(0).from_asset(format!("models/{file}"))
                    ),
                );
            }
            commands.insert_resource(DungeonAssets {assets});
            println!("Finished loading, going ingame lols :^)");
            next_state.set(AppState::Menu);
        },
        Err(err) => {
            // Thrown an error files nicht gefunden
            eprintln!("Error: Assets not found/couldnt be loaded. {} \n assets.rs", err);
            panic!()
        }
    }
}