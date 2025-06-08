use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::alien;
use crate::alien_projectile;
use crate::capsule;
use crate::fire;
use crate::game_audio;
use crate::level_indicator;
use crate::player;
use crate::projectile;
use crate::resolution;
use crate::star_field;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            alien::AlienPlugin,
            alien_projectile::AlienProjectilePlugin,
            capsule::CapsulePlugin,
            fire::FirePlugin,
            game_audio::GameAudioPlugin,
            level_indicator::LevelIndicatorPlugin,
            resolution::ResolutionPlugin,
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
            star_field::StarFieldPlugin,
        ))
        .add_systems(Startup, setup_scene)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new());
    }
}
fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
