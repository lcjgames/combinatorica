use bevy::prelude::*;

use crate::camera::MainCamera;
use crate::state::AppState;

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(display_background));
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(display_title));
    }
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

fn display_title(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(
        TextBundle::from_section(
            "Combinatorica",
            TextStyle {
                font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                font_size: 100.0,
                color: Color::ALICE_BLUE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            // position: UiRect {
            //     bottom: Val::Px(5.0),
            //     right: Val::Px(15.0),
            //     ..default()
            // },
            ..default()
        }),
    );
}
