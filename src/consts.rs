use bevy::prelude::*;

pub const P16: Vec2 = Vec2 { x: 16.0, y: 16.0 };

pub const SCALE_FACTOR: f32 = 2.0;
pub const WINDOW_SIZE: Vec2 = Vec2 { x: 220.0, y: 320.0 };
// pub const IN_BOUNDS: Rect = {
//     let margin = 32.0;
//     let radius = Vec2 {
//         x: (WINDOW_SIZE.x / 2.0) + margin,
//         y: (WINDOW_SIZE.y / 2.0) + margin,
//     };
//     let min = Vec2 {
//         x: -radius.x,
//         y: -radius.y,
//     };
//     let max = radius;
//     Rect { min, max }
// };
pub const IN_BOUNDS: Rect = {
    let margin = 32.0;
    let radius = Vec2 { x: 32.0, y: 32.0 };
    let min = Vec2 {
        x: -radius.x,
        y: -radius.y,
    };
    let max = radius;
    Rect { min, max }
};

pub const PLAYER_SIZE: Vec2 = Vec2 { x: 32.0, y: 32.0 };
pub const PLAYER_SPEED: f32 = 2.2;
