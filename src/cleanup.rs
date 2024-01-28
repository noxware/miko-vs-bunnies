use std::time::Duration;

use bevy::prelude::*;

pub struct CleanupPlugin;

impl Plugin for CleanupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cleanup_by_timers);
    }
}

#[derive(Component)]
pub struct CleanupTimer(Timer);

impl CleanupTimer {
    pub fn new(secs: f32) -> Self {
        Self(Timer::from_seconds(secs, TimerMode::Once))
    }
}

pub fn cleanup_by_timers(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut CleanupTimer)>,
) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(Duration::from_secs_f32(time.delta_seconds()));
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
