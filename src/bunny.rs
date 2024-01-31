use bevy::prelude::*;

use crate::{
    animation::{AnimationBundle, AnimationRange, ChangeAnimationEvent},
    enemy::Enemy,
    state::InGameState,
};

const IDLE: AnimationRange = AnimationRange::new(0, 0);
const WALK: AnimationRange = AnimationRange::new(1, 4);
const ANIMATION_TIMING: f32 = 0.1;

#[derive(Component)]
pub struct Bunny;

pub struct BunnyPlugin;

use crate::state::AppState;

impl Plugin for BunnyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_bunny)
            .add_systems(Update, animate_bunny.run_if(in_state(InGameState::Running)));
    }
}

fn spawn_bunny(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("bunny.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::splat(32.), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        AnimationBundle::new(IDLE, ANIMATION_TIMING),
        Enemy::default(),
        Bunny,
    ));
}

fn animate_bunny(
    query: Query<(Entity, &Enemy), With<Bunny>>,
    mut ev_change_animation: EventWriter<ChangeAnimationEvent>,
) {
    for (entity, enemy) in &query {
        let mut animation = ChangeAnimationEvent {
            entity,
            range: IDLE,
            secs: ANIMATION_TIMING,
        };

        if enemy.movement_status.is_chasing() {
            animation.range = WALK;
        }

        ev_change_animation.send(animation);
    }
}
