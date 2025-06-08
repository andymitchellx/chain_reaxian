use bevy::prelude::*;

use crate::alien_projectile::PlayerKilledEvent;
use crate::capsule::CapsuleCollisionEvent;
use crate::level_indicator::LevelCompletedEvent;
use crate::level_indicator::ScoreManager;
use crate::projectile;
use crate::resolution;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player).add_systems(
            Update,
            (update_player, capsule_collision, reset_when_killed),
        );
        app.add_event::<PlayerShootEvent>();
    }
}

#[derive(Component)]
pub struct Player {
    //provides cooldown for shooting so we don't just shoot a bullet every frame
    pub shoot_timer: f32,
    pub dead: bool,
    pub main_gun_projectiles: i32,
    pub side_gun_projectiles: i32,
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
            dead: false,
            main_gun_projectiles: 1,
            side_gun_projectiles: 0,
        },
    ));
}

const SPEED: f32 = 200.;
const BULLET_SPEED: f32 = 400.;
const SHOOT_COOLDOWN: f32 = 0.9;

#[derive(Event, Debug)]
pub struct PlayerShootEvent {}

fn update_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<resolution::Resolution>,
    mut events: EventWriter<PlayerShootEvent>,
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
        events.write(PlayerShootEvent {});
        player.shoot_timer = SHOOT_COOLDOWN;
        spawn_one_missile(
            &mut commands,
            &asset_server,
            &resolution,
            &transform,
            player.main_gun_projectiles,
        );
        spawn_two_missiles(
            &mut commands,
            &asset_server,
            &resolution,
            &transform,
            player.side_gun_projectiles,
        );
    }
}

const MAX_SIDE_BULLETS: i32 = 6;

fn capsule_collision(
    mut capsule_collision_events: EventReader<CapsuleCollisionEvent>,
    mut player_query: Query<&mut Player>,
    mut level_completed_events: EventWriter<LevelCompletedEvent>,
) {
    let mut player = player_query.single_mut().unwrap();
    for _ in capsule_collision_events.read() {
        if player.side_gun_projectiles < MAX_SIDE_BULLETS {
            if player.main_gun_projectiles > player.side_gun_projectiles {
                player.side_gun_projectiles += 1;
            } else {
                player.main_gun_projectiles += 1;
            }
        }

        level_completed_events.write(LevelCompletedEvent {});
    }
}

const PRIMARY_GUN_HEIGHT: f32 = 25.0;
const BULLET_HEIGHT: f32 = 12.0;

fn spawn_one_missile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    resolution: &Res<resolution::Resolution>,
    transform: &Mut<'_, Transform>,
    num_missiles: i32,
) {
    let bullet_texture: Handle<Image> = asset_server.load("images/chain.png");
    let mut y_pos = transform.translation.y + PRIMARY_GUN_HEIGHT;

    for _ in 0..num_missiles {
        commands.spawn((
            Sprite {
                image: bullet_texture.clone(),
                ..Default::default()
            },
            Transform::from_xyz(transform.translation.x, y_pos, transform.translation.z)
                .with_scale(Vec3::splat(resolution.pixel_ratio)),
            projectile::Projectile {
                speed: BULLET_SPEED,
            },
        ));

        y_pos -= BULLET_HEIGHT;
    }
}

const GUN_WIDTH: f32 = 20.0;

fn spawn_two_missiles(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    resolution: &Res<resolution::Resolution>,
    transform: &Mut<'_, Transform>,
    num_missiles: i32,
) {
    let bullet_texture: Handle<Image> = asset_server.load("images/chain.png");
    let mut y_pos = transform.translation.y + PRIMARY_GUN_HEIGHT;

    for _ in 0..num_missiles {
        commands.spawn((
            Sprite {
                image: bullet_texture.clone(),
                ..Default::default()
            },
            Transform::from_xyz(
                transform.translation.x - GUN_WIDTH,
                y_pos,
                transform.translation.z,
            )
            .with_scale(Vec3::splat(resolution.pixel_ratio)),
            projectile::Projectile {
                speed: BULLET_SPEED,
            },
        ));

        commands.spawn((
            Sprite {
                image: bullet_texture.clone(),
                ..Default::default()
            },
            Transform::from_xyz(
                transform.translation.x + GUN_WIDTH,
                y_pos,
                transform.translation.z,
            )
            .with_scale(Vec3::splat(resolution.pixel_ratio)),
            projectile::Projectile {
                speed: BULLET_SPEED,
            },
        ));

        y_pos -= BULLET_HEIGHT;
    }
}

fn reset_when_killed(
    mut player_killed_events: EventReader<PlayerKilledEvent>,
    mut player_query: Query<&mut Player>,
    mut events: EventWriter<LevelCompletedEvent>,
    mut score_manager: ResMut<ScoreManager>,
) {
    let mut player = player_query.single_mut().unwrap();
    for _ in player_killed_events.read() {
        score_manager.curr_level = 0;
        player.main_gun_projectiles = 1;
        player.side_gun_projectiles = 0;
        events.write(LevelCompletedEvent {});
    }
}
