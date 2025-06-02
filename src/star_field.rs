use bevy::prelude::*;

use crate::resolution;

pub struct StarFieldPlugin;

impl Plugin for StarFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_star_field);
    }
}

//width and height represent the amount of aliens horizontally and vertically which we wish to spawn
const WIDTH: f32 = 64.0;
const HEIGHT: f32 = 64.0;
const ZINDEX: f32 = -10.0;

fn setup_star_field(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    let star_field_texture = asset_server.load("images/star_field_atlas.png");
    let num_x_tiles: i32 = (resolution.screen_dimensions.x / WIDTH).floor() as i32 + 1;
    let num_y_tiles: i32 = (resolution.screen_dimensions.y / HEIGHT).floor() as i32 + 1;


    for x in -num_x_tiles..num_x_tiles {
        for y in -num_y_tiles..num_x_tiles {
            let position = Vec3::new(x as f32 * WIDTH,y as f32 * HEIGHT, ZINDEX);
            let rotation_radians: f32 = ((x + y) as f32 % 4 as f32 * 90.0).to_radians();
            let mut transform: Transform = Transform::from_translation(position)
                    .with_scale(Vec3::splat(resolution.pixel_ratio));
            transform.rotate(Quat::from_rotation_z(rotation_radians));

            commands.spawn((
                Sprite {
                    image: star_field_texture.clone(),
                    ..default()
                },
                transform,
            ));
        }
    }
}
