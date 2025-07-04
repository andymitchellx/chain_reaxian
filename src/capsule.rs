use bevy::prelude::*;
use rand::Rng;

use crate::player;
use crate::projectile::AlienKilledEvent;
use crate::resolution;

pub struct CapsulePlugin;

const CAPSULE_PCT: f32 = 4.0;
const CAPSULE_RADIUS: f32 = 24.;

impl Plugin for CapsulePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_capsule_counter).add_systems(
            Update,
            (spawn_capsules, update_capsules, update_capsule_interactions),
        );
        app.add_event::<CapsuleCollisionEvent>();
        app.add_event::<CapsuleReleasedEvent>();
    }
}

#[derive(Component)]
struct CapsuleCounter {
    num_capsules: i32,
}

#[derive(Component)]
pub struct Capsule {
    pub speed: f32,
}

#[derive(Event, Debug)]
pub struct CapsuleReleasedEvent {}

fn setup_capsule_counter(mut commands: Commands) {
    commands.spawn(CapsuleCounter { num_capsules: 0 });
}

// The maximum number of capsules on the screen at one time.
const MAX_CAPSULES: i32 = 1;

fn spawn_capsules(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut alien_killed_events: EventReader<AlienKilledEvent>,
    mut events: EventWriter<CapsuleReleasedEvent>,
    resolution: Res<resolution::Resolution>,
    mut counter_query: Query<&mut CapsuleCounter>,
) {
    let mut rng = rand::thread_rng();
    for event in alien_killed_events.read() {
        let mut counter = counter_query.single_mut().unwrap();
        if counter.num_capsules >= MAX_CAPSULES {
            return;
        }

        let pct = rng.gen_range(0.0..100.0);
        if pct < CAPSULE_PCT {
            let capsule_image = asset_server.load("images/orange_capsule.png");
            counter.num_capsules += 1;
            events.write(CapsuleReleasedEvent {});
            commands.spawn((
                Sprite {
                    image: capsule_image,
                    ..Default::default()
                },
                Transform::from_xyz(event.location.x, event.location.y, 5.0)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                Capsule { speed: 120.0 },
            ));
        }
    }
}

//move the capsule
fn update_capsules(
    mut commands: Commands,
    mut capsule_query: Query<(Entity, &Capsule, &mut Transform)>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
    mut counter_query: Query<&mut CapsuleCounter>,
) {
    for (entity, capsule, mut transform) in capsule_query.iter_mut() {
        transform.translation.y -= capsule.speed * time.delta_secs();
        if transform.translation.y.abs() > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();
            let mut counter = counter_query.single_mut().unwrap();
            counter.num_capsules -= 1;
        }
    }
}

#[derive(Event, Debug)]
pub struct CapsuleCollisionEvent {}

fn update_capsule_interactions(
    mut player_query: Query<(&mut player::Player, &Transform)>,
    mut capsule_query: Query<(Entity, &Transform), With<Capsule>>,
    mut commands: Commands,
    mut events: EventWriter<CapsuleCollisionEvent>,
    mut counter_query: Query<&mut CapsuleCounter>,
) {
    for (_, player_transform) in player_query.iter_mut() {
        for (capsule_entity, capsule_transform) in capsule_query.iter_mut() {
            let capsule_pos = Vec2::new(
                capsule_transform.translation.x,
                capsule_transform.translation.y,
            );
            let player_pos = Vec2::new(
                player_transform.translation.x,
                player_transform.translation.y,
            );
            if Vec2::distance(player_pos, capsule_pos) < CAPSULE_RADIUS {
                //best to not despawn in the query but the warning doesn't break the game so I don't mind too much
                commands.entity(capsule_entity).despawn();
                events.write(CapsuleCollisionEvent {});
                let mut counter = counter_query.single_mut().unwrap();
                counter.num_capsules -= 1;
            }
        }
    }
}
