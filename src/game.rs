use bevy::prelude::*;

use crate::alien;
use crate::player;
use crate::projectile;
use crate::resolution;
use crate::star_field;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            alien::AlienPlugin,
            resolution::ResolutionPlugin,
            player::PlayerPlugin,
            projectile::ProjectilePlugin,
            star_field::StarFieldPlugin,
        ))
        .add_systems(Startup, setup_scene);
    }
}
fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
