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

#[derive(Component)]
struct ChooseButton;

#[derive(Component)]
struct ChosenPart;

fn display(mut commands: Commands, asset_server: Res<AssetServer>) {
    crate::log::console_log!("SHIP EDITOR!");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Screen(AppState::ShipEditor))
        .with_children(|screen| {
            screen
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(70.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|upper_screen| {
                    upper_screen
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|left_side| {
                            //TODO: owned parts
                        });
                    upper_screen
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(70.0), Val::Percent(100.0)),
                                ..default()
                            },
                            color: Color::GREEN.into(),
                            ..default()
                        })
                        .with_children(|right_side| {
                            right_side
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                        flex_direction: FlexDirection::ColumnReverse,
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .with_children(|first_column| {
                                    first_column
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.0),
                                                    Val::Percent(50.0),
                                                ),
                                                flex_direction: FlexDirection::Column,
                                                ..default()
                                            },
                                            color: Color::RED.into(),
                                            ..default()
                                        })
                                        .with_children(|cockpit_node| {
                                            spawn_choose_button(
                                                cockpit_node,
                                                "Cockpit",
                                                &asset_server,
                                            );
                                        });
                                    first_column
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.0),
                                                    Val::Percent(50.0),
                                                ),
                                                flex_direction: FlexDirection::Column,
                                                ..default()
                                            },
                                            color: Color::RED.into(),
                                            ..default()
                                        })
                                        .with_children(|wings_node| {
                                            spawn_choose_button(wings_node, "Wings", &asset_server);
                                        });
                                });
                            right_side
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                                        flex_direction: FlexDirection::ColumnReverse,
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .with_children(|second_column| {
                                    second_column
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.0),
                                                    Val::Percent(50.0),
                                                ),
                                                flex_direction: FlexDirection::Column,
                                                ..default()
                                            },
                                            color: Color::RED.into(),
                                            ..default()
                                        })
                                        .with_children(|engine_node| {
                                            spawn_choose_button(
                                                engine_node,
                                                "Engine",
                                                &asset_server,
                                            );
                                        });
                                    second_column
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                size: Size::new(
                                                    Val::Percent(100.0),
                                                    Val::Percent(50.0),
                                                ),
                                                flex_direction: FlexDirection::Column,
                                                ..default()
                                            },
                                            color: Color::RED.into(),
                                            ..default()
                                        })
                                        .with_children(|lasergun_node| {
                                            spawn_choose_button(
                                                lasergun_node,
                                                "Lasergun",
                                                &asset_server,
                                            );
                                        });
                                });
                        });
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

fn spawn_choose_button(
    node: &mut ChildBuilder,
    name: &'static str,
    asset_server: &Res<AssetServer>,
) {
    node.spawn_bundle(ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(150.0)),
            margin: UiRect::new(Val::Percent(10.0), Val::Auto, Val::Auto, Val::Percent(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        color: Color::ALICE_BLUE.into(),
        ..default()
    })
    .insert(ChooseButton)
    .with_children(|button| {
        button.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::BLUE.into(),
            ..default()
        });
        button.spawn_bundle(
            TextBundle::from_section(
                name,
                TextStyle {
                    font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                    font_size: 10.0,
                    color: Color::DARK_GRAY,
                },
            )
            .with_text_alignment(TextAlignment::CENTER),
        );
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
