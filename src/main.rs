use bevy::prelude::*;

pub mod alien;
pub mod capsule;
pub mod game;
pub mod player;
pub mod projectile;
pub mod resolution;
pub mod star_field;

fn main() {
    App::new()
        .add_plugins((
            //list of plugins added to the game
            DefaultPlugins
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
