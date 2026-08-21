use bevy::{prelude::*};
use avian3d::prelude::*;

use crate::{AppState, enemy::Enemy, player::{Player, PlayerState}};
// Idee: Komplett weg gekuppelt
// Jedes Entity was schaden bekommen kann bekommt ein health, 
// und immer wenn einem entity damage bekommt dann wird das über damage events gemacht.

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DamageMessage>();
        app.add_systems(Update, (apply_damage, check_player_attack_hits).run_if(in_state(AppState::InGame)));
    }
}



#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}
impl Health {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max
        }
    }
}

#[derive(Message)]
pub struct DamageMessage {
    pub target: Entity,
    pub amount: f32
}

// Bro 1 Uhr nachts ich brauche so lang dafür
pub fn check_player_attack_hits(
    player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    spatial_query: SpatialQuery,
    mut damage_events: MessageWriter<DamageMessage>,
    mut gizmos: Gizmos,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    let Ok((player_entiy, player_transform, player)) = player_query.single() else { return;};

    // Wenn der Spieler nicht gerade angreift
    if keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) {
        
    } else {
        return;
    }
    // Größe der Trefferzone
    let size = Vec3::new(1.2, 1.0, 1.5); // Breite, Höhe, Tiefe
    let attack_shape = Collider::cuboid(1.2, 1.1, 1.5);

    // Position der Hitbox ein wenig vom Spieler entfernt
    let hit_position = player_transform.translation - player_transform.forward() * 1.0;

    gizmos.primitive_3d(
        &Cuboid::new(size.x, size.y, size.z),
        Isometry3d::new(hit_position, player_transform.rotation),
        Color::WHITE,
    );

    // Alle hits in der shape, avain ist schon toll
    let hits = spatial_query.shape_intersections(
        &attack_shape, 
        hit_position, 
        player_transform.rotation, 
        &SpatialQueryFilter::from_excluded_entities([player_entiy]));

    for hit_entity in hits {
        if enemy_query.contains(hit_entity) {// Hier müsste dann falls auch Kisten oder so getroffen werden das geändert werden
            damage_events.write(DamageMessage { 
                target: hit_entity, amount: 25.0 
            });
        }
    }
}

fn apply_damage(mut commands: Commands, mut damage_message: MessageReader<DamageMessage>, mut health_query: Query<(Entity, &mut Health)>) {
    for message in damage_message.read() {
        if let Ok((entity, mut health)) = health_query.get_mut(message.target) {
            health.current -= message.amount;
            println!("Entität {:?} hat noch {:.1} HP", entity, health.current);

            if health.current <= 0.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}