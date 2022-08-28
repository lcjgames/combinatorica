use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::parts::*;
use crate::ship::*;
use crate::state::*;

pub struct ShipEditor;

impl Plugin for ShipEditor {
    fn build(&self, app: &mut App) {
        app.add_event::<PartSelectionEvent>()
            .add_system_set(SystemSet::on_enter(AppState::ShipEditor).with_system(display))
            .add_system_set(SystemSet::on_update(AppState::ShipEditor).with_system(cancel_button))
            .add_system_set(SystemSet::on_update(AppState::ShipEditor).with_system(ok_button))
            .add_system_set(SystemSet::on_update(AppState::ShipEditor).with_system(choose_button))
            .add_system_set(SystemSet::on_exit(AppState::ShipEditor).with_system(screen_cleanup));
    }
}

#[derive(Component)]
struct CancelButton;

#[derive(Component)]
struct OkButton;

#[derive(Component)]
struct ChooseButton(PartType);

#[derive(Component)]
struct ChosenPart(PartType);

fn display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ship: Res<BuildingShip>,
    owned_parts: Res<OwnedParts>,
) {
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
                            color: Color::NONE.into(),
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
                                            color: Color::NONE.into(),
                                            ..default()
                                        })
                                        .with_children(|cockpit_node| {
                                            spawn_choose_button(
                                                cockpit_node,
                                                PartType::Cockpit,
                                                &asset_server,
                                                &owned_parts.get_image(
                                                    PartType::Cockpit,
                                                    ship.cockpit_index,
                                                ),
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
                                            color: Color::NONE.into(),
                                            ..default()
                                        })
                                        .with_children(|wings_node| {
                                            spawn_choose_button(
                                                wings_node,
                                                PartType::Wings,
                                                &asset_server,
                                                &owned_parts
                                                    .get_image(PartType::Wings, ship.wings_index),
                                            );
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
                                            color: Color::NONE.into(),
                                            ..default()
                                        })
                                        .with_children(|engine_node| {
                                            spawn_choose_button(
                                                engine_node,
                                                PartType::Engine,
                                                &asset_server,
                                                &owned_parts
                                                    .get_image(PartType::Engine, ship.engine_index),
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
                                            color: Color::NONE.into(),
                                            ..default()
                                        })
                                        .with_children(|lasergun_node| {
                                            spawn_choose_button(
                                                lasergun_node,
                                                PartType::Lasergun,
                                                &asset_server,
                                                &owned_parts.get_image(
                                                    PartType::Lasergun,
                                                    ship.lasergun_index,
                                                ),
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
                        .insert(OkButton)
                        .with_children(|button| {
                            button.spawn_bundle(
                                TextBundle::from_section(
                                    "Create",
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
    part_type: PartType,
    asset_server: &Res<AssetServer>,
    image: &String,
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
    .insert(ChooseButton(part_type))
    .with_children(|button| {
        button.spawn_bundle(ImageBundle {
            image: asset_server.load(image).into(),
            style: Style {
                size: Size::new(Val::Auto, Val::Percent(80.0)),
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        });
        button.spawn_bundle(
            TextBundle::from_section(
                format!("{:?}", part_type),
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

fn ok_button(
    mut state: ResMut<State<AppState>>,
    mut button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<OkButton>),
    >,
    mut ship: ResMut<BuildingShip>,
    mut owned_parts: ResMut<OwnedParts>,
    mut fleet: ResMut<Fleet>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::GRAY.into(),
            Interaction::None => Color::ALICE_BLUE.into(),
            Interaction::Clicked => {
                fleet.0.push(Ship {
                    wings_sprite: owned_parts.get_image(PartType::Wings, ship.wings_index),
                    cockpit_sprite: owned_parts.get_image(PartType::Cockpit, ship.cockpit_index),
                    strength: ship.strength(&owned_parts),
                    active: false,
                    destroyed: false,
                });
                owned_parts.cockpit = owned_parts
                    .cockpit
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &ship.cockpit_index)
                    .map(|(_, x)| x.clone())
                    .collect();
                owned_parts.engine = owned_parts
                    .engine
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &ship.engine_index)
                    .map(|(_, x)| x.clone())
                    .collect();
                owned_parts.wings = owned_parts
                    .wings
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &ship.wings_index)
                    .map(|(_, x)| x.clone())
                    .collect();
                owned_parts.lasergun = owned_parts
                    .lasergun
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i != &ship.lasergun_index)
                    .map(|(_, x)| x.clone())
                    .collect();
                *ship = BuildingShip::default();
                state.set(AppState::FleetEditor).unwrap();
                Color::GREEN.into()
            }
        }
    }
}

fn choose_button(
    mut state: ResMut<State<AppState>>,
    mut part_selection_event_writer: EventWriter<PartSelectionEvent>,
    mut button_query: Query<
        (&Interaction, &mut UiColor, &ChooseButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, choose_button) in button_query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::GRAY.into(),
            Interaction::None => Color::ALICE_BLUE.into(),
            Interaction::Clicked => {
                part_selection_event_writer.send(PartSelectionEvent(choose_button.0));
                state.set(AppState::PartSelection).unwrap();
                Color::GREEN.into()
            }
        }
    }
}
