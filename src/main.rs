#![feature(core_intrinsics)]
#![feature(iter_advance_by)]

extern crate core;

use bevy::prelude::*;

mod battle;
use battle::*;

mod camera;
use camera::*;

mod combinatorics;

mod fleet_editor;
use fleet_editor::*;

// mod loading;
// use loading::*;

#[macro_use]
mod log;

mod main_menu;
use main_menu::*;

mod part_selection;
use part_selection::*;

mod parts;
use parts::*;

mod ship;
use ship::*;

mod ship_editor;
use ship_editor::*;

mod shop;
use shop::*;

mod state;
use state::*;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    console_log!("Starting Game!");
    App::new()
        .add_state(AppState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(Battle)
        .add_plugin(FleetEditor)
        // .add_plugin(Loading)
        .add_plugin(MainMenu)
        .add_plugin(PartSelection)
        .add_plugin(Parts)
        .add_plugin(ShipEditor)
        .add_plugin(ShipPlugin)
        .add_plugin(Shop)
        .add_startup_system(spawn_main_camera)
        .add_startup_system(display_background)
        .run();
}

fn display_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let window_size = Vec3::new(window.width(), window.height(), 0.0);
    let start_position = -window_size / 2.0;

    let bg = asset_server.load("spaceshooter/Backgrounds/black.png");
    let size = 256;
    let n_horizontal = window.width().round() as i32 / size + 2;
    let n_vertical = window.height().round() as i32 / size + 2;

    for i in 0..n_horizontal {
        for j in 0..n_vertical {
            let random_stuff = ((i + j) % 4) as f32;
            let angle = random_stuff * std::f32::consts::PI / 2.0;
            let transform = Transform::from_translation(
                start_position + Vec3::new((i * size) as f32, (j * size) as f32, 0.0),
            )
            .with_rotation(Quat::from_axis_angle(Vec3::Z, angle));
            commands.spawn_bundle(SpriteBundle {
                texture: bg.clone(),
                transform,
                ..default()
            });
        }
    }
}
