use bevy::prelude::*;

use crate::{consts::*, player::*};

pub fn handle_keyboard_move_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Controlled>>,
) {
    const SPEED: f32 = PLAYER_SPEED;

    let mut delta = Vec2::new(0.0, 0.0);

    if keyboard_input.pressed(KeyCode::Left) {
        delta.x -= SPEED;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        delta.x += SPEED;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        delta.y += SPEED;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        delta.y -= SPEED;
    }

    let mut player_transform = query.single_mut();
    player_transform.translation += delta.extend(0.0);
}
