use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::parts::*;
use crate::ship::*;
use crate::state::*;
use crate::Sprites;

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
    sprites: Res<Sprites>,
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
                            let font = sprites.font.clone();
                            let small_style = TextStyle {
                                font: font.clone(),
                                font_size: 25.0,
                                color: Color::GRAY,
                            };
                            let big_style = TextStyle {
                                font: font.clone(),
                                font_size: 30.0,
                                color: Color::ALICE_BLUE,
                            };
                            left_side.spawn_bundle(
                                TextBundle::from_sections([
                                    TextSection::new(
                                        "Preview\n",
                                        TextStyle {
                                            font,
                                            font_size: 30.0,
                                            color: Color::GRAY,
                                        },
                                    ),
                                    TextSection::new("\nBase Strength:\n", small_style.clone()),
                                    TextSection::new(
                                        format!("{:.2}\n", ship.base_strength(&owned_parts)),
                                        big_style.clone(),
                                    ),
                                    TextSection::new("\nBonus Strength:\n", small_style.clone()),
                                    TextSection::new(
                                        format!("{:.2}\n", ship.bonus_strength(&owned_parts)),
                                        big_style.clone(),
                                    ),
                                    TextSection::new("\nPossibilities:\n", small_style.clone()),
                                    TextSection::new(
                                        format!("{}\n", owned_parts.possibilities()),
                                        big_style.clone(),
                                    ),
                                    TextSection::new("\nTotal Strength:\n", small_style.clone()),
                                    TextSection::new(
                                        format!("{:.2}\n", ship.strength(&owned_parts).0),
                                        big_style.clone(),
                                    ),
                                ])
                                .with_text_alignment(TextAlignment::BOTTOM_CENTER)
                                .with_style(Style {
                                    align_self: AlignSelf::Center,
                                    max_size: Size {
                                        width: Val::Px(300.0),
                                        height: Val::Undefined,
                                    },
                                    ..default()
                                }),
                            );
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
                                                &sprites,
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
                                                &sprites,
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
                                                &sprites,
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
                                                &sprites,
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
                                        font: sprites.font.clone(),
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
                                        font: sprites.font.clone(),
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
    sprites: &Res<Sprites>,
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
                    font: sprites.font.clone(),
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
    use rand::prelude::*;
    let mut rng = thread_rng();

    for (interaction, mut color) in button_query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::GRAY.into(),
            Interaction::None => Color::ALICE_BLUE.into(),
            Interaction::Clicked => {
                fleet.0.push(Ship {
                    wings_sprite: owned_parts.get_image(PartType::Wings, ship.wings_index),
                    flipped_wing_spite: owned_parts.get_flipped_wing(ship.wings_index),
                    cockpit_sprite: owned_parts.get_image(PartType::Cockpit, ship.cockpit_index),
                    strength: ship.strength(&owned_parts),
                    active: false,
                    destroyed: false,
                    pilot_name: PILOT_NAMES[rng.gen_range(0..PILOT_NAMES.len())],
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
