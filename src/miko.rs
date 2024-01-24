use crate::animation::{AnimationBundle, AnimationRange, ChangeAnimationEvent};
use bevy::prelude::*;

const IDLE: AnimationRange = AnimationRange::new(0, 0);
const ATTACK: AnimationRange = AnimationRange::new(1, 1);
const WALK: AnimationRange = AnimationRange::new(2, 4);
const ANIMATION_TIMING: f32 = 0.1;
const WALK_SPEED: f32 = 50.0;
const RUN_SPEED: f32 = 150.0;

// TODO: Move this component out.
// TODO: Also change animation speed.
#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct Speed {
    pub value: f32,
}

pub struct MikoPlugin;

impl Plugin for MikoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_miko);
        app.add_systems(Update, (animate_miko, move_miko, miko_run));
    }
}

#[derive(Component)]
pub struct Miko;

fn spawn_miko(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("miko.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(32.), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(WALK.first),
            ..default()
        },
        AnimationBundle::new(WALK, ANIMATION_TIMING),
        Speed::default(),
        Miko,
    ));
}

fn animate_miko(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut TextureAtlasSprite), With<Miko>>,
    mut ev_change_animation: EventWriter<ChangeAnimationEvent>,
) {
    for (entity, mut sprite) in &mut query {
        if keyboard_input.pressed(KeyCode::Z) {
            ev_change_animation.send(ChangeAnimationEvent {
                entity,
                range: ATTACK,
                secs: ANIMATION_TIMING,
            });
        } else if keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::Right)
            || keyboard_input.pressed(KeyCode::Up)
            || keyboard_input.pressed(KeyCode::Down)
        {
            ev_change_animation.send(ChangeAnimationEvent {
                entity,
                range: WALK,
                secs: ANIMATION_TIMING,
            });
        } else {
            ev_change_animation.send(ChangeAnimationEvent {
                entity,
                range: IDLE,
                secs: ANIMATION_TIMING,
            });
        }

        if keyboard_input.pressed(KeyCode::Left) {
            sprite.flip_x = true;
        } else if keyboard_input.pressed(KeyCode::Right) {
            sprite.flip_x = false;
        }
    }
}

fn move_miko(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Miko>>,
) {
    if keyboard_input.pressed(KeyCode::Z) {
        return;
    }

    for (mut transform, speed) in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * speed.value * time.delta_seconds();
        }
    }
}

fn miko_run(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Speed, With<Miko>>) {
    for mut speed in &mut query {
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed.value = RUN_SPEED;
        } else {
            speed.value = WALK_SPEED;
        }
    }
}
