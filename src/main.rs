use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod animation;
mod bunny;
mod camera;
mod cleanup;
mod common;
mod enemy;
mod miko;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            camera::CameraPlugin,
            miko::MikoPlugin,
            bunny::BunnyPlugin,
            enemy::EnemyPlugin,
            animation::AnimationPlugin,
            world::WorldPlugin,
            cleanup::CleanupPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
