use bevy::prelude::*;

use rand::seq::IteratorRandom;

use crate::alien::Alien;
use crate::alien::Dead;
use crate::player;
use crate::resolution;

pub struct AlienProjectilePlugin;

impl Plugin for AlienProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cooldown).add_systems(
            Update,
            (
                fire_projectile,
                update_alien_projectiles,
                update_player_interactions,
            ),
        );
        app.add_event::<PlayerKilledEvent>();
        app.add_event::<AlienShootEvent>();
    }
}

#[derive(Component)]
struct AlienProjectileCooldown {
    shoot_timer: f32,
}

const SHOOT_COOLDOWN: f32 = 1.2;
const BULLET_SPEED: f32 = 240.;

#[derive(Event, Debug)]
pub struct PlayerKilledEvent {}

#[derive(Event, Debug)]
pub struct AlienShootEvent {}

#[derive(Component)]
pub struct AlienProjectile {
    pub speed: f32,
}

fn setup_cooldown(mut commands: Commands) {
    commands.spawn(AlienProjectileCooldown { shoot_timer: 0. });
}

//move the projectiles
fn update_alien_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &AlienProjectile, &mut Transform)>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
) {
    for (entity, alien_projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation.y -= alien_projectile.speed * time.delta_secs();
        if transform.translation.y > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

const BULLET_RADIUS: f32 = 10.;

fn update_player_interactions(
    mut player_query: Query<(&mut player::Player, &Transform)>,
    mut alien_projectile_query: Query<(Entity, &Transform), With<AlienProjectile>>,
    mut commands: Commands,
    mut events: EventWriter<PlayerKilledEvent>,
) {
    let (mut player, player_transform) = player_query.single_mut().unwrap();
    for (alien_projectile_entity, alien_projectile_transform) in alien_projectile_query.iter_mut() {
        let alien_projectile_pos = Vec2::new(
            alien_projectile_transform.translation.x,
            alien_projectile_transform.translation.y,
        );
        let player_pos = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );
        if Vec2::distance(player_pos, alien_projectile_pos) < BULLET_RADIUS {
            player.dead = true;
            commands.entity(alien_projectile_entity).despawn();
            events.write(PlayerKilledEvent {});
        }
    }
}

fn fire_projectile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cooldown_query: Query<&mut AlienProjectileCooldown>,
    alien_query: Query<&mut Transform, (With<Alien>, Without<Dead>)>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
    mut events: EventWriter<AlienShootEvent>,
) {
    let mut cooldown = cooldown_query.single_mut().unwrap();
    let mut rng = rand::thread_rng();
    cooldown.shoot_timer -= time.delta_secs();

    if cooldown.shoot_timer <= 0. {
        if let Some(transform) = alien_query.iter().choose(&mut rng) {
            events.write(AlienShootEvent {});
            cooldown.shoot_timer = SHOOT_COOLDOWN;
            let bullet_texture: Handle<Image> = asset_server.load("images/chain.png");
            commands.spawn((
                Sprite {
                    image: bullet_texture,
                    ..Default::default()
                },
                Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    transform.translation.z,
                )
                .with_scale(Vec3::splat(resolution.pixel_ratio)),
                AlienProjectile {
                    speed: BULLET_SPEED,
                },
            ));
        }
    }
}
