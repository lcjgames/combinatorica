use crate::Sprites;
use bevy::prelude::*;

use crate::state::*;

pub struct MainMenu;

impl Plugin for MainMenu {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(display_title))
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(display_buttons))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(activate_buttons))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(screen_cleanup));
    }
}

fn display_title(mut commands: Commands, sprites: Res<Sprites>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Combinatorica",
                TextStyle {
                    font: sprites.font.clone(),
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
        )
        .insert(Screen(AppState::MainMenu));
}

fn display_buttons(mut commands: Commands, sprites: Res<Sprites>) {
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
                    font: sprites.font.clone(),
                    font_size: 35.0,
                    color: Color::DARK_GRAY,
                },
            ));
        })
        .insert(Screen(AppState::MainMenu));
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
