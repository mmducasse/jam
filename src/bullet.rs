use std::mem::size_of;

use bevy::prelude::*;

use crate::{consts::*, physics::Velocity, team::*};

#[derive(Clone, Copy)]
pub enum BulletType {
    PlayerSmall,
    PlayerCannon,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Bundle)]
pub struct BulletBundle {
    sprite: SpriteSheetBundle,
    velocity: Velocity,
    team: Team,
    bullet: Bullet,
}

impl BulletBundle {
    pub fn new(
        pos: Vec2,
        angle: f32,
        type_: BulletType,
        team_id: TeamId,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let speed = get_speed_of_bullet_type(type_);
        let velocity = Vec2::from_angle(angle) * speed;

        let texture_handle = asset_server.load("bullets.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, P16, 4, 4, None, None);
        println!("{}", size_of::<TextureAtlas>());

        let atlas_index = get_sprite_idx_of_bullet_type(type_);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        Self {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(atlas_index),
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            velocity: Velocity(velocity),
            team: Team(team_id),
            bullet: Bullet,
        }
    }
}

fn get_speed_of_bullet_type(type_: BulletType) -> f32 {
    use BulletType::*;
    match type_ {
        PlayerSmall => 2.0,
        PlayerCannon => 4.0,
    }
}

fn get_sprite_idx_of_bullet_type(type_: BulletType) -> usize {
    use BulletType::*;
    match type_ {
        PlayerSmall => 0,
        PlayerCannon => 1,
    }
}
