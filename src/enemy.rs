use bevy::prelude::*;

use crate::miko::Miko;
use crate::state::InGameState;

const SPEED: f32 = 45.0;
const DETECTION_RADIUS: f32 = 100.0;
const NEAR_ENOUGH: f32 = 10.0;

pub enum EnemyMovementStatus {
    Idle,
    Moving,
    ReachedTarget,
}

impl EnemyMovementStatus {
    pub fn is_chasing(&self) -> bool {
        match self {
            EnemyMovementStatus::Idle => false,
            EnemyMovementStatus::Moving => true,
            EnemyMovementStatus::ReachedTarget => true,
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub movement_status: EnemyMovementStatus,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            movement_status: EnemyMovementStatus::Idle,
        }
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, goto_player.run_if(in_state(InGameState::Running)));
    }
}

fn goto_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Enemy), Without<Miko>>,
    player_query: Query<&Transform, With<Miko>>,
) {
    for (mut transform, mut sprite, mut enemy) in &mut query {
        let player_transform = player_query.single();
        let distance = transform.translation.distance(player_transform.translation);

        if distance < NEAR_ENOUGH {
            enemy.movement_status = EnemyMovementStatus::ReachedTarget;
            continue;
        }

        if distance < DETECTION_RADIUS {
            enemy.movement_status = EnemyMovementStatus::Moving;
            let direction = player_transform.translation - transform.translation;
            let direction = direction.normalize_or_zero();

            transform.translation += direction * SPEED * time.delta_seconds();
            if direction.x > 0.0 {
                sprite.flip_x = true;
            } else {
                sprite.flip_x = false;
            }
        } else {
            enemy.movement_status = EnemyMovementStatus::Idle;
        }
    }
}
