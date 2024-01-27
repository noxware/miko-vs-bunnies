use bevy::prelude::*;

use crate::enemy::Enemy;

#[derive(Component)]
pub struct Bunny;

pub struct BunnyPlugin;

impl Plugin for BunnyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bunny);
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
        Enemy,
        Bunny,
    ));
}
