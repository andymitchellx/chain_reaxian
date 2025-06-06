use bevy::prelude::*;

use crate::alien_projectile::PlayerKilledEvent;
use crate::capsule::CapsuleCollisionEvent;
use crate::level_indicator::LevelCompletedEvent;
use crate::projectile;
use crate::resolution;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player).add_systems(
            Update,
            (update_player, capsule_collision, reset_when_killed),
        );
    }
}

#[derive(Component)]
pub struct Player {
    //provides cooldown for shooting so we don't just shoot a bullet every frame
    pub shoot_timer: f32,
    pub projectile_type: ProjectileType,
    pub dead: bool,
    pub level: i32,
}

pub enum ProjectileType {
    SingleShot,
    DoubleShot,
    TripleShot,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    let player_sheet = asset_server.load("images/player.png");
    commands.spawn((
        Sprite {
            image: player_sheet,
            ..Default::default()
        },
        Transform::from_xyz(
            0.,
            -(resolution.screen_dimensions.y * 0.5) + (resolution.pixel_ratio * 25.0),
            0.,
        )
        .with_scale(Vec3::splat(resolution.pixel_ratio)),
        Player {
            shoot_timer: 0.,
            projectile_type: ProjectileType::SingleShot,
            dead: false,
            level: 1,
        },
    ));
}

const SPEED: f32 = 200.;
const BULLET_SPEED: f32 = 400.;
const SHOOT_COOLDOWN: f32 = 0.9;

fn update_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<resolution::Resolution>,
) {
    let (mut player, mut transform) = player_query.single_mut().unwrap();

    //the input which the player is pressing for the horizontal axis
    let mut horizontal = 0.;

    if keys.pressed(KeyCode::KeyA) {
        horizontal += -1.;
    }
    if keys.pressed(KeyCode::KeyD) {
        horizontal += 1.;
    }
    //move player
    transform.translation.x += horizontal * time.delta_secs() * SPEED;

    //confine player
    let left_bound = -resolution.screen_dimensions.x * 0.5;
    let right_bound = resolution.screen_dimensions.x * 0.5;

    if transform.translation.x > right_bound {
        transform.translation.x = right_bound;
    }
    if transform.translation.x < left_bound {
        transform.translation.x = left_bound;
    }

    player.shoot_timer -= time.delta_secs();

    if keys.pressed(KeyCode::Space) && player.shoot_timer <= 0. {
        player.shoot_timer = SHOOT_COOLDOWN;
        match player.projectile_type {
            ProjectileType::DoubleShot => {
                spawn_two_missiles(commands, asset_server, resolution, transform)
            }
            ProjectileType::TripleShot => {
                spawn_one_missile(&mut commands, &asset_server, &resolution, &transform);
                spawn_two_missiles(commands, asset_server, resolution, transform);
            }
            _ => spawn_one_missile(&mut commands, &asset_server, &resolution, &transform),
        }
    }
}

fn capsule_collision(
    mut capsule_collision_events: EventReader<CapsuleCollisionEvent>,
    mut player_query: Query<&mut Player>,
    mut level_completed_events: EventWriter<LevelCompletedEvent>,
) {
    let mut player = player_query.single_mut().unwrap();
    for _ in capsule_collision_events.read() {
        match player.projectile_type {
            ProjectileType::DoubleShot => {
                player.projectile_type = ProjectileType::TripleShot;
                level_completed_events.write(LevelCompletedEvent {});
            }
            ProjectileType::TripleShot => {
                level_completed_events.write(LevelCompletedEvent {});
            }
            _ => player.projectile_type = ProjectileType::DoubleShot,
        }
    }
}

const PRIMARY_GUN_HEIGHT: f32 = 25.0;

fn spawn_one_missile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    resolution: &Res<resolution::Resolution>,
    transform: &Mut<'_, Transform>,
) {
    let bullet_texture: Handle<Image> = asset_server.load("images/chain.png");
    commands.spawn((
        Sprite {
            image: bullet_texture,
            ..Default::default()
        },
        Transform::from_xyz(
            transform.translation.x,
            transform.translation.y + PRIMARY_GUN_HEIGHT,
            transform.translation.z,
        )
        .with_scale(Vec3::splat(resolution.pixel_ratio)),
        projectile::Projectile {
            speed: BULLET_SPEED,
        },
    ));
}

const GUN_WIDTH: f32 = 20.0;

fn spawn_two_missiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
    transform: Mut<'_, Transform>,
) {
    let bullet_texture: Handle<Image> = asset_server.load("images/chain.png");
    commands.spawn((
        Sprite {
            image: bullet_texture.clone(),
            ..Default::default()
        },
        Transform::from_xyz(
            transform.translation.x - GUN_WIDTH,
            transform.translation.y,
            transform.translation.z,
        )
        .with_scale(Vec3::splat(resolution.pixel_ratio)),
        projectile::Projectile {
            speed: BULLET_SPEED,
        },
    ));

    commands.spawn((
        Sprite {
            image: bullet_texture,
            ..Default::default()
        },
        Transform::from_xyz(
            transform.translation.x + GUN_WIDTH,
            transform.translation.y,
            transform.translation.z,
        )
        .with_scale(Vec3::splat(resolution.pixel_ratio)),
        projectile::Projectile {
            speed: BULLET_SPEED,
        },
    ));
}

fn reset_when_killed(
    mut player_killed_events: EventReader<PlayerKilledEvent>,
    mut player_query: Query<&mut Player>,
    mut events: EventWriter<LevelCompletedEvent>,
) {
    let mut player = player_query.single_mut().unwrap();
    for _ in player_killed_events.read() {
        player.projectile_type = ProjectileType::SingleShot;
        player.level = 0;
        events.write(LevelCompletedEvent {});
    }
}
