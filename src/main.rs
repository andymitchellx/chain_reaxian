use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

pub mod alien;
pub mod alien_layouts;
pub mod alien_projectile;
pub mod capsule;
pub mod fire;
pub mod game;
pub mod level_indicator;
pub mod player;
pub mod projectile;
pub mod resolution;
pub mod star_field;
pub mod widget;

fn main() {
    App::new()
        .add_plugins((
            //list of plugins added to the game
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Chain Reaxian"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(612., 612.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            game::GamePlugin,
        ))
        .run();
}
