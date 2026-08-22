use bevy::{prelude::*};

use crate::{AppState, assets::DungeonAssets};

#[derive(Component, Clone, Copy, Default)]
pub struct MenuUI;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu);

        app.add_systems(OnExit(AppState::Menu), cleaup_menu);
    }
}

// Wiederverwendbarer Button
fn menu_button(label: &'static str) -> impl Scene { // lifetime wegen dem child label bsn
    bsn! {
        Button
        Node {
            width: px(200.),
            height: px(65.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(Color::srgb(0.2, 0.2, 0.4))

        // Verfärbung beim hovern
        on(|press: On<Pointer<Click>>, mut next_state: ResMut<NextState<AppState>>| {
            next_state.set(AppState::InGame)
        })

        on(|hover: On<Pointer<Over>>, mut bg: Query<&mut BackgroundColor>| {
            if let Ok(mut bgc) = bg.single_mut() {
                *bgc = BackgroundColor(Color::srgb(0.2, 0.9, 0.4));
            }
        })

        Children [(
            Text(label)
            TextFont {
                font_size: px(30.)
            }
            TextColor(Color::WHITE)
            TextShadow
        )]
    }
}

fn setup_menu(mut commands: Commands, dungeon_assets: Res<DungeonAssets>) {
    commands.spawn_scene(bsn! {
        Camera2d
        MenuUI
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: px(30),
            row_gap: px(30)
        }
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15))

        // Die Elemente als Kinder
        Children [(
            Text("Dungeon Game")
            TextFont{font_size: px(55.0)}
            TextColor(Color::WHITE)
        ),
        (
            Text("Move with WASD")
            TextFont{font_size: px(25.0)}
            TextColor(Color::WHITE)
        ),
        (
            Text("Attack with left mouse button")
            TextFont{font_size: px(25.0)}
            TextColor(Color::WHITE)
        ),
        menu_button("Start Game")
        ]
    });
}

fn cleaup_menu(query: Query<Entity, With<MenuUI>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}