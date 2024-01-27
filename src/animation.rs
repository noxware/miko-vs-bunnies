use std::time::Duration;

use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate, change_animation))
            .add_event::<ChangeAnimationEvent>();
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

#[derive(Event)]
pub struct ChangeAnimationEvent {
    pub entity: Entity,
    pub range: AnimationRange,
    pub secs: f32,
}

fn change_animation(
    mut ev_change_animation: EventReader<ChangeAnimationEvent>,
    mut query: Query<(
        &mut AnimationRange,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for ev in ev_change_animation.read() {
        let (mut range, mut timer, mut sprite) = query
            .get_mut(ev.entity)
            .expect("Failed to get AnimationRange and TextureAtlasSprite");

        if *range != ev.range {
            sprite.index = ev.range.first;
        }

        *range = ev.range;
        timer.0.set_duration(Duration::from_secs_f32(ev.secs));
    }
}
