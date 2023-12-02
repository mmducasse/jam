mod anim;
mod background;
mod consts;
mod input;
mod player;

use anim::*;
use background::*;
use bevy::{prelude::*, window::WindowResolution};
use input::*;
use player::*;

use consts::*;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            WINDOW_SIZE.x * SCALE_FACTOR,
                            WINDOW_SIZE.y * SCALE_FACTOR,
                        )
                        .with_scale_factor_override(SCALE_FACTOR as f64),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_sprite,
                handle_keyboard_move_input,
                update_background,
            ),
        )
        .add_systems(PostUpdate, keep_in_bounds)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerBundle::new(
        Vec2::new(0.0, -WINDOW_SIZE.y / 4.0),
        &asset_server,
        &mut texture_atlases,
    ));
    commands.spawn(BackgroundBundle::new(&asset_server));
}
