use bevy::prelude::*;

use std::fs;
use std::io;

pub struct TestWorldPlugin;

impl Plugin for TestWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_test_world);
    }
}

fn setup_test_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    match get_model_filenames("./assets/models") {
        Ok(files) => {
            let mut index = 0;
            for file in files {
                commands.spawn((
                    WorldAssetRoot(asset_server.load(
                        GltfAssetLabel::Scene(0).from_asset(format!("models/{file}"))
                    )),
                    Transform::from_xyz(2. * index as f32, 0.0, 0.0)
                ));
                index += 1;
            }
            println!("lksadlökd")
        },
        Err(err) => eprintln!("Error: {}", err)
    }
}

pub fn get_model_filenames(folder_path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(folder_path)?;

    let filenames = entries
        .filter_map(|entry| entry.ok()) // filtert alle fehler
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter_map(|path| path.file_name()?.to_str().map(|s| s.to_string()))
        .collect();

    Ok(filenames)
}