use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod animation;
mod camera;
mod miko;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(camera::CameraPlugin)
        .add_plugins(miko::MikoPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}
