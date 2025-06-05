use bevy::prelude::*;

use crate::alien_layouts::*;
use crate::alien_projectile::PlayerKilledEvent;
use crate::resolution;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_aliens, setup_wave))
            .add_systems(Update, (update_aliens, manage_alien_logic, player_killed));
    }
}

#[derive(Component)]
pub struct Alien {
    pub dead: bool,
    pub original_position: Vec3,
    pub alien_type: AlienType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AlienType {
    Worker,
    Soldier,
    Queen,
    Empty,
}

//a marker component to prevent querying any dead aliens in our updates after they have already died
#[derive(Component)]
pub struct Dead;

//controls the behavior of our aliens
#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
    //we increment the aliens vertically when this is true once
    pub shift_aliens_down: bool,
    //the distance the closest alien to the edge is from the boundary so that we can correct it to be confined within the boundary
    pub dist_from_boundary: f32,
    //the game will reset when this is triggered
    pub reset: bool,
    pub speed: f32,
    pub prev_alien_count: i32,
}

//width and height represent the amount of aliens horizontally and vertically which we wish to spawn
const HORIZ_SPACING: f32 = 22.;
const VERT_SPACING: f32 = 50.0;
const SPEED: f32 = 35.0;
const SPEED_INCREMENT: f32 = 12.0;
const ALIEN_SHIFT_AMOUNT: f32 = 16.;
const ZINDEX: f32 = 15.0;
const VERT_OFFSET: f32 = 70.0;

//spawn our aliens
fn setup_aliens(mut commands: Commands) {
    commands.insert_resource(AlienManager {
        reset: false,
        dist_from_boundary: 0.,
        shift_aliens_down: false,
        direction: 1.,
        speed: SPEED,
        prev_alien_count: 99,
    });
}

#[allow(clippy::needless_range_loop)]
fn setup_wave(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    let worker_texture = asset_server.load("images/alien_worker.png");
    let soldier_texture = asset_server.load("images/alien_soldier.png");
    let queen_texture = asset_server.load("images/alien_queen.png");
    let half_width = ALIEN_COLS as f32 * HORIZ_SPACING * 0.5;

    for row in 0..ALIEN_ROWS {
        for col in 0..ALIEN_COLS {
            let mask_value = DEFAULT_MASK[row][col];
            let alien_type;
            let mut alien_image: Handle<Image> = worker_texture.clone();
            match mask_value {
                1 => {
                    alien_type = AlienType::Worker;
                    alien_image = worker_texture.clone();
                }
                2 => {
                    alien_type = AlienType::Soldier;
                    alien_image = soldier_texture.clone();
                }
                3 => {
                    alien_type = AlienType::Queen;
                    alien_image = queen_texture.clone();
                }
                _ => alien_type = AlienType::Empty,
            }

            if alien_type != AlienType::Empty {
                let position = Vec3::new(
                    col as f32 * HORIZ_SPACING - half_width,
                    row as f32 * VERT_SPACING + VERT_OFFSET,
                    ZINDEX,
                );
                // let position = Vec3::new(col as f32 * SPACING, row as f32 * SPACING, ZINDEX)
                // - (Vec3::X * ALIEN_COLS as f32 * 0.5);
                //  - (Vec3::Y * ALIEN_ROWS as f32 * 1.0)
                //  + (Vec3::Y * resolution.screen_dimensions.y * 0.5);
                commands.spawn((
                    Sprite {
                        image: alien_image,
                        ..default()
                    },
                    Transform::from_translation(position)
                        .with_scale(Vec3::splat(resolution.pixel_ratio)),
                    Alien {
                        original_position: position,
                        dead: false,
                        alien_type: AlienType::Worker,
                    },
                ));
            }
        }
    }
}

fn update_aliens(
    mut commands: Commands,
    //only query aliens that are still alive
    mut alien_query: Query<(Entity, &Alien, &mut Transform, &mut Visibility), Without<Dead>>,
    mut alien_manager: ResMut<AlienManager>,
    mut events: EventWriter<PlayerKilledEvent>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
) {
    let margin = resolution.screen_dimensions.x * 0.5 - (resolution.pixel_ratio * 25.0);
    let mut alien_alive = false;
    let mut alien_count = 0;
    for (entity, alien, mut transform, mut visibility) in alien_query.iter_mut() {
        //delta_seconds makes it so our aliens move at the same speed regardless of framerate; delta_seconds() gives the time between each frame.
        transform.translation.x +=
            time.delta_secs() * alien_manager.direction * alien_manager.speed;
        if transform.translation.x.abs() > margin {
            alien_manager.shift_aliens_down = true;
            alien_manager.dist_from_boundary =
                margin * alien_manager.direction - transform.translation.x;
        }

        if alien.dead {
            commands.entity(entity).insert(Dead {});
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }

        //if the aliens have made it out of the bottom of the screen we have lost the game and should reset
        if transform.translation.y < -resolution.screen_dimensions.y * 0.5 + 70. {
            alien_manager.reset = true;
            events.write(PlayerKilledEvent {});
        }

        alien_alive = true;
        alien_count += 1;
    }

    if !alien_alive {
        alien_manager.reset = true;
    }

    if (alien_count < 30 && alien_manager.prev_alien_count >= 30)
        || (alien_count < 20 && alien_manager.prev_alien_count >= 20)
        || (alien_count < 10 && alien_manager.prev_alien_count >= 10)
        || (alien_count < 3 && alien_manager.prev_alien_count >= 3)
    {
        alien_manager.speed += SPEED_INCREMENT;
    }

    alien_manager.prev_alien_count = alien_count;
}

fn manage_alien_logic(
    mut commands: Commands,
    mut alien_query: Query<(Entity, &mut Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,
) {
    if alien_manager.shift_aliens_down {
        //reverse direction and move aliens downward
        alien_manager.shift_aliens_down = false;
        alien_manager.direction *= -1.;
        for (_entity, _alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.dist_from_boundary;
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;
        }
    }

    if alien_manager.reset {
        alien_manager.reset = false;
        alien_manager.direction = 1.;
        for (entity, mut alien, mut transform) in alien_query.iter_mut() {
            transform.translation = alien.original_position;
            if alien.dead {
                //revive our alien from the dead unit pool
                alien.dead = false;
                commands.entity(entity).remove::<Dead>();
            }
        }
    }
}

fn player_killed(
    mut player_killed_events: EventReader<PlayerKilledEvent>,
    mut alien_manager: ResMut<AlienManager>,
) {
    for _ in player_killed_events.read() {
        alien_manager.reset = true;
        alien_manager.speed = SPEED;
    }
}
