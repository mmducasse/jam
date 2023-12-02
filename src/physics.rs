use bevy::prelude::*;

use crate::consts::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

pub fn move_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0.extend(0.0);
    }
}

#[derive(Component)]
pub struct DieOutside;

pub fn die_outside(mut commands: Commands, query: Query<(Entity, &Transform, With<DieOutside>)>) {
    for (entity, transform, _) in query.iter() {
        if false == IN_BOUNDS.contains(transform.translation.truncate()) {
            commands.entity(entity).despawn();
        }
    }
}
