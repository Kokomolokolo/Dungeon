use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct PlayerHealthBar;

pub fn setup_player_hud(mut commands: Commands) {
    commands.spawn_scene(
        bsn! { 
            Node {
                position_type: PositionType::Absolute,
                left: px(20.),
                top: px(20.),
                width: px(200),
                height: px(20),
            }
            BackgroundColor(Color::WHITE)
            ZIndex(i32::MAX)

            Children [
                PlayerHealthBar
                Node {
                    width: Val::Percent(100.0), // wird per System aktualisiert
                    height: Val::Percent(100.0),
                }
                BackgroundColor(Color::WHITE)
            ]
        }
        
    );
}