use bevy::prelude::*;

use crate::consts::*;

const SCROLL_SPEED_Y: f32 = 2.0;

#[derive(Bundle)]
pub struct BackgroundBundle {
    pub sprite: SpriteBundle,
    pub background: Background,
}

#[derive(Component)]
pub struct Background;

impl BackgroundBundle {
    pub fn new(asset_server: &AssetServer) -> Self {
        let texture_handle = asset_server.load("background.png");

        let sprite = SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -10.0)),
            ..default()
        };

        Self {
            sprite: sprite,
            background: Background,
        }
    }
}

pub fn update_background(mut query: Query<&mut Transform, With<Background>>) {
    for mut transform in &mut query {
        transform.translation.y -= SCROLL_SPEED_Y;

        if transform.translation.y < -WINDOW_SIZE.y {
            transform.translation.y += WINDOW_SIZE.y;
        }
    }
}
