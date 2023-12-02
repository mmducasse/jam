use bevy::prelude::*;

use crate::anim::*;

#[derive(Component)]
pub struct Controlled;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub sprite: SpriteSheetBundle,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub controlled: Controlled,
}

impl PlayerBundle {
    pub fn new(
        pos: Vec2,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load("player.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 1, None, None);

        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let animation_indices = AnimationIndices::from_series(0, 2);

        PlayerBundle {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(animation_indices.first),
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },

            controlled: Controlled,
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        }
    }
}
