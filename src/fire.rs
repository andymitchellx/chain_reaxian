use bevy::prelude::*;

use crate::alien::{self, AlienType, Dead};
use crate::projectile::AlienKilledEvent;
use crate::resolution;

pub struct FirePlugin;

const FIRE_RADIUS: f32 = 10.;
const FIRE_LIFESPAN: f32 = 2.;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_fire, update_fire, update_fire_interactions));
    }
}

#[derive(Component)]
pub struct Fire {
    pub time_remaining: f32,
}

fn spawn_fire(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut alien_killed_events: EventReader<AlienKilledEvent>,
    resolution: Res<resolution::Resolution>,
) {
    for event in alien_killed_events.read() {
        if event.alien_type == AlienType::Worker {
            let fire_image = asset_server.load("images/fire.png");
            commands.spawn((
                Sprite {
                    image: fire_image,
                    ..Default::default()
                },
                Transform::from_xyz(event.location.x, event.location.y, 5.0)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                Fire {
                    time_remaining: FIRE_LIFESPAN,
                },
            ));
        }
    }
}

fn update_fire(
    mut commands: Commands,
    mut fire_query: Query<(Entity, &mut Fire)>,
    time: Res<Time>,
) {
    for (entity, mut fire) in fire_query.iter_mut() {
        fire.time_remaining -= time.delta_secs();

        if fire.time_remaining < 0. {
            commands.entity(entity).despawn();
        }
    }
}

fn update_fire_interactions(
    mut alien_query: Query<(&mut alien::Alien, &Transform), Without<Dead>>,
    mut fire_query: Query<&Transform, With<Fire>>,
    mut events: EventWriter<AlienKilledEvent>,
) {
    for (mut alien, alien_transform) in alien_query.iter_mut() {
        for fire_transform in fire_query.iter_mut() {
            let fire_pos = Vec2::new(fire_transform.translation.x, fire_transform.translation.y);

            let alien_pos = Vec2::new(alien_transform.translation.x, alien_transform.translation.y);

            if Vec2::distance(alien_pos, fire_pos) < FIRE_RADIUS {
                alien.dead = true;
                //commands.entity(alien_entity).despawn();
                events.write(AlienKilledEvent {
                    alien_type: alien.alien_type,
                    location: alien_pos,
                });
            }
        }
    }
}
