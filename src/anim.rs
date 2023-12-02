use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub count: usize,
}

impl AnimationIndices {
    pub fn from_series(first: usize, count: usize) -> Self {
        if count == 0 {
            panic!("AnimationIndices must have at least one element.");
        }

        Self { first, count }
    }

    pub fn last(&self) -> usize {
        self.first + self.count - 1
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index >= indices.last() {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
