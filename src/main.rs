use bevy::prelude::*;

mod camera;
use camera::*;

mod combinatorics;

mod fleet_editor;
use fleet_editor::*;

mod input;
use input::*;

mod loading;
use loading::*;

#[macro_use]
mod log;

mod main_menu;
use main_menu::*;

mod movement;
use movement::*;

mod ship;
use ship::*;

mod sprites;
use sprites::*;

mod state;
use state::*;

pub fn spawn(
    mut commands: Commands,
    sprite_sheets: Res<SpriteSheets>,
    ground_tiles: Res<GroundTiles>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(AnimatedSpriteBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: sprite_sheets.player_fishy.clone(),
                sprite: TextureAtlasSprite {
                    index: IDLE_FISH.start(),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 48.0, 0.0)),
                ..default()
            },
            sprite_position: IDLE_FISH,
            animation_timer: AnimationTimer(Timer::from_seconds(0.1, true)),
        })
        .insert(Velocity::default())
        .insert(Controls::default());
    //characters test (to be removed)
    commands.spawn_bundle(AnimatedSpriteBundle {
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprite_sheets.player_orcy.clone(),
            sprite: TextureAtlasSprite {
                index: DYING_BACKWARD_FISH.start(),
                flip_x: true,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(200.0, -160.0, 0.0)),
            ..default()
        },
        sprite_position: DYING_BACKWARD_FISH,
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, true)),
    });
    commands.spawn_bundle(AnimatedSpriteBundle {
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprite_sheets.player_pescy.clone(),
            sprite: TextureAtlasSprite {
                index: DYING_FORWARD_FISH.start(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-200.0, -160.0, 0.0)),
            ..default()
        },
        sprite_position: DYING_FORWARD_FISH,
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, true)),
    });
    //tiles test (to be removed)
    for i in 0..17 {
        for j in 0..5 {
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: ground_tiles.metal.clone(),
                sprite: TextureAtlasSprite {
                    index: *TileIndex::new(j, i),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    -600.0 + (i as f32) * 40.0,
                    200.0 - (j as f32) * 40.0,
                    0.0,
                )),
                ..default()
            });
        }
    }
    for i in 0..10 {
        let index = if i == 0 {
            *TOPBOTTOM_LEFT_LEDGE
        } else if i == 9 {
            *TOPBOTTOM_RIGHT_LEDGE
        } else {
            *TOPBOTTOM_MIDDLE_LEDGE
        };
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: ground_tiles.metal.clone(),
            sprite: TextureAtlasSprite { index, ..default() },
            transform: Transform::from_translation(Vec3::new(-120.0 + (i as f32) * 32.0, 0.0, 0.0)),
            ..default()
        });
    }
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    console_log!("Starting Game!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FleetEditor)
        .add_plugin(Loading)
        .add_plugin(MainMenu)
        .add_plugin(ShipPlugin)
        .add_startup_system(spawn_main_camera)
        .add_startup_system(display_background)
        .add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn))
        .add_system_set(SystemSet::on_update(AppState::Battle).with_system(animate_sprite))
        .add_system_set(SystemSet::on_update(AppState::Battle).with_system(input_handling))
        .add_system_set(SystemSet::on_update(AppState::Battle).with_system(movement))
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
