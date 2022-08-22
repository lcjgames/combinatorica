use bevy::prelude::*;

use crate::camera::MainCamera;
use crate::state::AppState;

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(display_background))
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(display_title))
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(display_buttons))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(activate_buttons));
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
            ..default()
        }),
    );
}

fn display_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::new(Val::Percent(10.0), Val::Auto, Val::Auto, Val::Percent(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::ALICE_BLUE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                    font_size: 35.0,
                    color: Color::DARK_GRAY,
                },
            ));
        });
}

pub fn activate_buttons(
    mut state: ResMut<State<AppState>>,
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::GRAY.into(),
            Interaction::None => Color::ALICE_BLUE.into(),
            Interaction::Clicked => {
                state.set(AppState::FleetEditor).unwrap();
                Color::ALICE_BLUE.into()
            }
        }
    }
}
