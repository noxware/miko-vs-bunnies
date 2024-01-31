use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod animation;
mod bunny;
mod camera;
mod cleanup;
mod common;
mod controller;
mod enemy;
mod miko;
mod state;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            state::StatePlugin,
            camera::CameraPlugin,
            miko::MikoPlugin,
            bunny::BunnyPlugin,
            enemy::EnemyPlugin,
            animation::AnimationPlugin,
            world::WorldPlugin,
            cleanup::CleanupPlugin,
            controller::ControllerPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
