use crate::animation::{AnimationBundle, AnimationRange, ChangeAnimationEvent};
use bevy::prelude::*;

const IDLE: AnimationRange = AnimationRange::new(0, 0);
const ATTACK: AnimationRange = AnimationRange::new(1, 1);
const WALK: AnimationRange = AnimationRange::new(2, 4);
const MAGIC: AnimationRange = AnimationRange::new(0, 3);
const ANIMATION_TIMING: f32 = 0.1;
const FAST_ANIMATION_TIMING: f32 = 0.06;
const WALK_SPEED: f32 = 50.0;
const RUN_SPEED: f32 = 100.0;

pub struct MikoPlugin;

impl Plugin for MikoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets)
            .add_systems(Startup, spawn_miko)
            .add_systems(
                Update,
                (animate_miko, move_miko, trigger_magic, release_magic),
            );
    }
}

#[derive(Resource)]
struct MikoHandles {
    miko_atlas: Handle<TextureAtlas>,
    magic_atlas: Handle<TextureAtlas>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let miko_atlas = {
        let texture_handle = asset_server.load("miko.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::splat(32.), 5, 1, None, None);
        texture_atlases.add(texture_atlas)
    };

    let magic_atlas = {
        let texture_handle = asset_server.load("fiush.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::splat(32.), 4, 1, None, None);
        texture_atlases.add(texture_atlas)
    };

    commands.insert_resource(MikoHandles {
        miko_atlas,
        magic_atlas,
    });
}

#[derive(Component)]
pub struct Miko;

fn spawn_miko(mut commands: Commands, handles: Res<MikoHandles>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: handles.miko_atlas.clone(),
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

#[derive(Component)]
struct Magic;

fn trigger_magic(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    handles: Res<MikoHandles>,
    miko_query: Query<(&Transform, &TextureAtlasSprite), With<Miko>>,
) {
    if keyboard_input.just_pressed(KeyCode::Z) {
        let (miko_transform, miko_sprite) = miko_query.single();
        let mut transform = miko_transform.clone();
        if miko_sprite.flip_x {
            transform.translation.x -= 32.0;
        } else {
            transform.translation.x += 32.0;
        }

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: handles.magic_atlas.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform,
                ..default()
            },
            AnimationBundle::new(MAGIC, 0.1),
            Magic,
        ));
    }
}

fn release_magic(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<Entity, With<Magic>>,
) {
    if keyboard_input.just_released(KeyCode::Z) {
        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}
