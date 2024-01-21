use bevy::prelude::*;

mod animation;
mod camera;
mod miko;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(camera::CameraPlugin)
        .add_plugins(miko::MikoPlugin)
        .add_plugins(animation::AnimationPlugin)
        .run();
}
