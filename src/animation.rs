use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate);
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnimationRange {
    pub first: usize,
    pub last: usize,
}

impl AnimationRange {
    pub const fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate(
    time: Res<Time>,
    mut query: Query<(
        &AnimationRange,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (range, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == range.last {
                range.first
            } else {
                sprite.index + 1
            };
        }
    }
}

#[derive(Bundle)]
pub struct AnimationBundle {
    range: AnimationRange,
    timer: AnimationTimer,
}

impl AnimationBundle {
    pub fn new(range: AnimationRange, secs: f32) -> Self {
        Self {
            range,
            timer: AnimationTimer(Timer::from_seconds(secs, TimerMode::Repeating)),
        }
    }
}
