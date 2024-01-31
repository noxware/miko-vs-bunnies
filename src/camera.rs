use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::miko::Miko;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, follow_miko);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.0,
            far: 1000.0,
            scaling_mode: ScalingMode::FixedVertical(32. * 5.),
            ..default()
        },
        ..default()
    },));
}

fn follow_miko(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    miko_query: Query<&Transform, (With<Miko>, Without<Camera>)>,
) {
    if let Ok(miko_transform) = miko_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation = miko_transform.translation;
    }
}
