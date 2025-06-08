use bevy::prelude::*;

use crate::widget;

pub struct LevelIndicatorPlugin;

#[derive(Component)]
struct LevelTextParent {}

#[derive(Component)]
struct LevelText {
    time_remaining: f32,
}

#[derive(Component)]
struct ScoreParent {}

#[derive(Resource)]
pub struct ScoreManager {
    pub curr_level: i32,
    pub max_level: i32,
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
        widget::ui_center_root("Level Indicator"),
        GlobalZIndex(2),
        LevelTextParent {},
    ));

    commands.spawn((
        widget::ui_center_root("Score"),
        GlobalZIndex(2),
        ScoreParent {},
    ));

    commands.insert_resource(ScoreManager {
        curr_level: 1,
        max_level: 1,
    });
}

const TIME_REMAINING: f32 = 1.7;

fn update_level_complete(
    mut commands: Commands,
    parent_query: Query<Entity, With<LevelTextParent>>,
    mut events: EventReader<LevelCompletedEvent>,
    mut score_manager: ResMut<ScoreManager>,
) {
    for _ in events.read() {
        for parent in parent_query.iter() {
            score_manager.curr_level += 1;
            if score_manager.curr_level > score_manager.max_level {
                score_manager.max_level = score_manager.curr_level;
            }

            let child = commands
                .spawn((
                    widget::large_text(format!(
                        "Level {} (Max: {})",
                        score_manager.curr_level, score_manager.max_level
                    )),
                    LevelText {
                        time_remaining: TIME_REMAINING,
                    },
                ))
                .id();

            commands.entity(parent).add_child(child);
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
