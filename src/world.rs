use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

const GRASS_TYPES_COUNT: usize = 4;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}

pub fn spawn_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("floor.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::splat(32.),
        GRASS_TYPES_COUNT,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut rng = StdRng::seed_from_u64(42);
    for x in -10..10 {
        for y in -10..10 {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(rng.gen_range(0..GRASS_TYPES_COUNT)),
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * 32.,
                    y as f32 * 32.,
                    -1.,
                )),
                ..default()
            });
        }
    }
}
