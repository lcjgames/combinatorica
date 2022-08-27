use bevy::prelude::*;

use crate::parts::*;
use crate::state::*;

pub struct ShipEditor;

impl Plugin for ShipEditor {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::ShipEditor).with_system(display))
            .add_system_set(SystemSet::on_update(AppState::ShipEditor).with_system(cancel_button))
            .add_system_set(SystemSet::on_exit(AppState::ShipEditor).with_system(screen_cleanup));
    }
}

#[derive(Component)]
struct CancelButton;

#[derive(Component)]
struct OkButton;

fn display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ship: ResMut<BuildingShip>,
    parts: Res<OwnedParts>,
) {
    crate::log::console_log!("SHIP EDITOR!");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Screen(AppState::ShipEditor))
        .with_children(|screen| {
            screen.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(70.0)),
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            });
            screen
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|lower_screen| {
                    lower_screen
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(75.0)),
                                margin: UiRect::new(
                                    Val::Percent(10.0),
                                    Val::Auto,
                                    Val::Auto,
                                    Val::Percent(10.0),
                                ),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            color: Color::ALICE_BLUE.into(),
                            ..default()
                        })
                        .insert(CancelButton)
                        .with_children(|button| {
                            button.spawn_bundle(
                                TextBundle::from_section(
                                    "Cancel",
                                    TextStyle {
                                        font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                                        font_size: 30.0,
                                        color: Color::DARK_GRAY,
                                    },
                                )
                                .with_text_alignment(TextAlignment::CENTER),
                            );
                        });
                });
        });
}

fn cancel_button(
    mut state: ResMut<State<AppState>>,
    mut button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<CancelButton>),
    >,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::GRAY.into(),
            Interaction::None => Color::ALICE_BLUE.into(),
            Interaction::Clicked => {
                state.set(AppState::FleetEditor).unwrap();
                Color::GREEN.into()
            }
        }
    }
}
