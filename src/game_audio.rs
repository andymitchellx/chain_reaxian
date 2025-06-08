use bevy::{audio::Volume, prelude::*};
use bevy_rustysynth::MidiAudio;

use crate::{
    alien::{ALIEN_SPEED_INCREMENT, INITIAL_ALIEN_SPEED, SpeedChangedEvent},
    alien_projectile::{AlienShootEvent, PlayerKilledEvent},
    capsule::{CapsuleCollisionEvent, CapsuleReleasedEvent},
    player::PlayerShootEvent,
    projectile::AlienKilledEvent,
};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_cooldown, play_music))
            .add_systems(
                Update,
                (
                    alien_killed,
                    alien_shoot,
                    capsule_collision,
                    capsule_released,
                    player_killed,
                    player_shoot,
                    speed_changed,
                    update_cooldowns,
                ),
            );
    }
}

#[derive(Component)]
struct AudioCooldowns {
    alien_killed_timer: f32,
    capsule_collision_timer: f32,
    capsule_release_timer: f32,
}

const ALIEN_KILLED_COOLDOWN: f32 = 0.3;
const CAPSULE_COLLISION_COOLDOWN: f32 = 0.8;
const CAPSULE_RELEASE_COOLDOWN: f32 = 0.8;

#[derive(Component)]
struct GameMusic;

fn play_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    let audio = asset_server.load::<MidiAudio>("sounds/background-music.mid");
    let volume = Volume::Linear(3.5);

    commands.spawn((
        AudioPlayer(audio),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume,
            ..default()
        },
        GameMusic {},
    ));
}

fn setup_cooldown(mut commands: Commands) {
    commands.spawn(AudioCooldowns {
        alien_killed_timer: 0.,
        capsule_collision_timer: 0.,
        capsule_release_timer: 0.,
    });
}

fn alien_killed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut alien_killed_events: EventReader<AlienKilledEvent>,
    mut cooldown_query: Query<&mut AudioCooldowns>,
) {
    let mut cooldown = cooldown_query.single_mut().unwrap();
    if cooldown.alien_killed_timer <= 0. {
        for _ in alien_killed_events.read() {
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/alienKilled.ogg"),
            ));

            cooldown.alien_killed_timer = ALIEN_KILLED_COOLDOWN;
        }
    }
}

fn capsule_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut alien_killed_events: EventReader<CapsuleCollisionEvent>,
    mut cooldown_query: Query<&mut AudioCooldowns>,
) {
    let mut cooldown = cooldown_query.single_mut().unwrap();
    if cooldown.capsule_collision_timer <= 0. {
        for _ in alien_killed_events.read() {
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/capsuleCollision.ogg"),
            ));

            cooldown.capsule_collision_timer = CAPSULE_COLLISION_COOLDOWN;
        }
    }
}

fn capsule_released(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut alien_killed_events: EventReader<CapsuleReleasedEvent>,
    mut cooldown_query: Query<&mut AudioCooldowns>,
) {
    let mut cooldown = cooldown_query.single_mut().unwrap();
    if cooldown.capsule_collision_timer <= 0. {
        for _ in alien_killed_events.read() {
            commands.spawn(AudioPlayer::new(
                asset_server.load("sounds/capsuleRelease.ogg"),
            ));

            cooldown.capsule_release_timer = CAPSULE_RELEASE_COOLDOWN;
        }
    }
}

fn player_killed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_killed_events: EventReader<PlayerKilledEvent>,
) {
    for _ in player_killed_events.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("sounds/playerKilled.ogg"),
        ));
    }
}

fn player_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_shoot_events: EventReader<PlayerShootEvent>,
) {
    for _ in player_shoot_events.read() {
        commands.spawn(AudioPlayer::new(
            asset_server.load("sounds/playerShoot.ogg"),
        ));
    }
}

fn alien_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut alien_shoot_events: EventReader<AlienShootEvent>,
) {
    for _ in alien_shoot_events.read() {
        commands.spawn(AudioPlayer::new(asset_server.load("sounds/alienShoot.ogg")));
    }
}

fn update_cooldowns(mut cooldown_query: Query<&mut AudioCooldowns>, time: Res<Time>) {
    let mut cooldown = cooldown_query.single_mut().unwrap();
    cooldown.alien_killed_timer -= time.delta_secs();
    cooldown.capsule_collision_timer -= time.delta_secs();
    cooldown.capsule_release_timer -= time.delta_secs();
}

const SPEED_FACTOR: f32 = 0.02;

fn speed_changed(
    mut events: EventReader<SpeedChangedEvent>,
    music_controller: Query<&AudioSink, With<GameMusic>>,
) {
    for speed_changed in events.read() {
        let Ok(sink) = music_controller.single() else {
            return;
        };

        let num_speed_increments =
            (speed_changed.speed - INITIAL_ALIEN_SPEED) / ALIEN_SPEED_INCREMENT;

        let new_speed = 1. + (num_speed_increments * SPEED_FACTOR);
        sink.set_speed(new_speed);
    }
}
