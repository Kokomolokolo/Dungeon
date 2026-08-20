use bevy::prelude::*;

use crate::{AppState, assets::DungeonAssets, player::{Player, PlayerAnimations, PlayerState}};

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        // Nur das Setup im Startup ausführen
        app.add_systems(Startup, setup_animation);
        
        app.add_systems(
            Update,
            (link_animation_graph, animate_player)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn setup_animation(mut commands: Commands, dungeon_assets: Res<DungeonAssets>, asset_server: Res<AssetServer>, mut graphs: ResMut<Assets<AnimationGraph>>) {
    // Animationen laden
    let static_clip: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(0).from_asset("models/player.glb"));
    let idle_clip: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(1).from_asset("models/player.glb"));
    let walk_clip: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(2).from_asset("models/player.glb"));
    let attack_clip: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(20).from_asset("models/player.glb"));
    let die_clip: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(10).from_asset("models/player.glb"));


    println!("{:?}", walk_clip);
    // Animationsgrapgen
    let mut graph = AnimationGraph::new();
    let static_node = graph.add_clip(static_clip, 1.0, graph.root);
    let idle_node = graph.add_clip(idle_clip, 1.0, graph.root);
    let walk_node = graph.add_clip(walk_clip, 1.0, graph.root);
    let attack_node = graph.add_clip(attack_clip, 1.0, graph.root);
    let die_node = graph.add_clip(die_clip, 1.0, graph.root);

    let graph_handle = graphs.add(graph);
    commands.insert_resource(PlayerAnimations {
        graph: graph_handle.clone(),
        idle: idle_node,
        walking: walk_node,
        attack: attack_node,
        die: die_node,
        pose: static_node
    });
}

// Verknüpft Graph mit AnimationPlayer
fn link_animation_graph(
    mut commands: Commands,
    animations: Res<PlayerAnimations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in &mut players {
        let mut transitions = AnimationTransitions::new();
        transitions.play(&mut player, animations.idle, std::time::Duration::ZERO).repeat();

        commands.entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}

fn animate_player(
    animations: Res<PlayerAnimations>,
    players: Query<&Player, Changed<Player>>, // Changed: Nur wenn sich player.state ändert
    mut anim_query: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    for player in &players {
        println!("{:?}", player.state);
        for (mut anim_player, mut transitions) in &mut anim_query {
            let target_node = match player.state {
                PlayerState::Idle => animations.idle,
                PlayerState::Walking => animations.walking,
                PlayerState::Attack => animations.attack,
                _ => animations.idle,
            };

            transitions
                .play(&mut anim_player, target_node, std::time::Duration::from_secs_f32(0.2))
                .repeat();
        }
    }
}

