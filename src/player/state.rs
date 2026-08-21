use avian3d::dynamics::rigid_body::LinearVelocity;
use bevy::{prelude::*, render::render_resource::CachedPipelineState::Ok};

use crate::player::{Player, PlayerAnimations, PlayerState::{self, Attack}};

// Nicht Alle state änderungen finden hier statt. Nur die Rückfälle von anderen States die priorisieren
pub fn update_player_state(
    query: Query<(&LinearVelocity, &mut Player)>
) {
    for (velocity, mut player) in query {
        // Entscheidet je nachdem was der Spieler gerade macht welche Animation als erstes Ausgeführt werden soll
        // 
        // 1. Priorität : Tod
        // Dann nichts mehr machen
        if player.state == PlayerState::Die {
            continue;
        }

        // Attakiert der Spieler
        if player.state == PlayerState::Attack {
            continue;
        }
        
        // Bewegt sich der Spieler?
        let is_moving = velocity.length_squared() > 0.1;

        let new_state = if is_moving {
            PlayerState::Walking
        } else {
            PlayerState::Idle
        };
        
        // Wichtig: Nur ändern wenn auch geändert werden muss
        if new_state != player.state {
            player.state = new_state;
        }
    }
}

pub fn check_attack_finished(
    player_query: Query<&mut Player>,
    anim_query: Query<&AnimationPlayer>,
    animations: Res<PlayerAnimations>,
) {
    for mut player in player_query {
        if player.state == PlayerState::Attack {
            for anim_player in anim_query {
                if let Some(active_anim) = anim_player.animation(animations.attack) {
                    if active_anim.is_finished() {
                        player.state = PlayerState::Idle;
                    }
                }
            }
        }
    }
}