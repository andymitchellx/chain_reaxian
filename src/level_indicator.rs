use bevy::prelude::*;

use crate::{player, widget};

pub struct LevelIndicatorPlugin;

#[derive(Component)]
struct LevelTextParent {}

#[derive(Component)]
struct LevelText {
    time_remaining: f32,
}

impl Plugin for LevelIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_parent_widget);
        app.add_systems(Update, (update_level_complete, destroy_level_complete));
        app.add_event::<LevelCompletedEvent>();
    }
}

#[derive(Event)]
pub struct LevelCompletedEvent {}

fn setup_parent_widget(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Level Indicator"),
        GlobalZIndex(2),
        LevelTextParent {},
    ));
}

const TIME_REMAINING: f32 = 1.7;

fn update_level_complete(
    mut commands: Commands,
    mut player_query: Query<&mut player::Player>,
    parent_query: Query<Entity, With<LevelTextParent>>,
    mut events: EventReader<LevelCompletedEvent>,
) {
    for _ in events.read() {
        for mut player in player_query.iter_mut() {
            for parent in parent_query.iter() {
                player.level += 1;

                let child = commands
                    .spawn((
                        widget::header(format!("Level {}", player.level)),
                        LevelText {
                            time_remaining: TIME_REMAINING,
                        },
                    ))
                    .id();

                commands.entity(parent).add_child(child);
            }
        }
    }
}

fn destroy_level_complete(
    mut commands: Commands,
    mut level_text_query: Query<(Entity, &mut LevelText)>,
    time: Res<Time>,
) {
    for (entity, mut level_text) in level_text_query.iter_mut() {
        level_text.time_remaining -= time.delta_secs();
        if level_text.time_remaining < 0. {
            commands.entity(entity).despawn();
        }
    }
}
