use bevy::prelude::*;

use crate::{assets::DungeonAssets, player::PlayerAnimations};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn setup_animation(mut commands: Commands, dungeon_assets: Res<DungeonAssets>, asset_server: Res<AssetServer>, mut graphs: ResMut<Assets<AnimationGraph>>) {
    // Animationen laden
    let idle_clip: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(0).from_asset("model/player.glb"));
    let walk_clip: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(1).from_asset("models/player.glb"));

    // Animationsgrapgen
    let mut graph = AnimationGraph::new();
    let idle_node = graph.add_clip(idle_clip, 1.0, graph.root);
    let walk_node = graph.add_clip(walk_clip, 1.0, graph.root);

    let graph_handle = graphs.add(graph);
    commands.insert_resource(PlayerAnimations {
        graph: graph_handle.clone(),
        idle: idle_node,
        walking: walk_node,
    });
}