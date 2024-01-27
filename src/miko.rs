use crate::animation::{AnimationBundle, AnimationRange, ChangeAnimationEvent};
use bevy::prelude::*;

const IDLE: AnimationRange = AnimationRange::new(0, 0);
const ATTACK: AnimationRange = AnimationRange::new(1, 1);
const WALK: AnimationRange = AnimationRange::new(2, 4);
const ANIMATION_TIMING: f32 = 0.1;
const FAST_ANIMATION_TIMING: f32 = 0.06;
const WALK_SPEED: f32 = 50.0;
const RUN_SPEED: f32 = 100.0;

pub struct MikoPlugin;

impl Plugin for MikoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_miko);
        app.add_systems(Update, (animate_miko, move_miko));
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
        Miko,
    ));
}

fn animate_miko(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut TextureAtlasSprite), With<Miko>>,
    mut ev_change_animation: EventWriter<ChangeAnimationEvent>,
) {
    for (entity, mut sprite) in &mut query {
        let mut animation = ChangeAnimationEvent {
            entity,
            range: IDLE,
            secs: ANIMATION_TIMING,
        };

        if keyboard_input.pressed(KeyCode::Z) {
            animation.range = ATTACK;
        } else if keyboard_input.pressed(KeyCode::Left)
            || keyboard_input.pressed(KeyCode::Right)
            || keyboard_input.pressed(KeyCode::Up)
            || keyboard_input.pressed(KeyCode::Down)
        {
            animation.range = WALK;
        }

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            animation.secs = FAST_ANIMATION_TIMING;
        }

        ev_change_animation.send(animation);

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
    mut query: Query<&mut Transform, With<Miko>>,
) {
    if keyboard_input.pressed(KeyCode::Z) {
        return;
    }

    for mut transform in &mut query {
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
            let speed = if keyboard_input.pressed(KeyCode::ShiftLeft) {
                RUN_SPEED
            } else {
                WALK_SPEED
            };

            direction = direction.normalize();
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}
