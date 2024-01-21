use crate::animation::{AnimationBundle, AnimationRange, ChangeAnimationEvent};
use bevy::prelude::*;

const IDLE: AnimationRange = AnimationRange::new(0, 0);
const ATTACK: AnimationRange = AnimationRange::new(1, 1);
const WALK: AnimationRange = AnimationRange::new(2, 4);
const ANIMATION_TIMING: f32 = 0.1;

pub struct MikoPlugin;

impl Plugin for MikoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_miko);
        app.add_systems(Update, move_miko);
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
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimationBundle::new(WALK, ANIMATION_TIMING),
        Miko,
    ));
}

fn move_miko(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut TextureAtlasSprite), With<Miko>>,
    mut ev_change_animation: EventWriter<ChangeAnimationEvent>,
) {
    for (entity, mut sprite) in &mut query {
        if keyboard_input.pressed(KeyCode::Left) {
            sprite.flip_x = true;
            ev_change_animation.send(ChangeAnimationEvent {
                entity,
                range: WALK,
                secs: ANIMATION_TIMING,
            });
        } else if keyboard_input.pressed(KeyCode::Right) {
            sprite.flip_x = false;
            ev_change_animation.send(ChangeAnimationEvent {
                entity,
                range: WALK,
                secs: ANIMATION_TIMING,
            });
        } else if keyboard_input.pressed(KeyCode::Z) {
            ev_change_animation.send(ChangeAnimationEvent {
                entity,
                range: ATTACK,
                secs: ANIMATION_TIMING,
            });
        } else {
            ev_change_animation.send(ChangeAnimationEvent {
                entity,
                range: IDLE,
                secs: ANIMATION_TIMING,
            });
        }
    }
}
