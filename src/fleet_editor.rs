use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::ship::*;
use crate::state::*;

pub struct FleetEditor;

impl Plugin for FleetEditor {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::FleetEditor).with_system(display_ships))
            .add_system_set(SystemSet::on_update(AppState::FleetEditor).with_system(activate_ships))
            .add_system_set(
                SystemSet::on_update(AppState::FleetEditor).with_system(go_button_interaction),
            )
            .add_system_set(
                SystemSet::on_update(AppState::FleetEditor).with_system(go_button_activation),
            )
            .add_system_set(
                SystemSet::on_update(AppState::FleetEditor).with_system(update_strength),
            )
            .add_system_set(SystemSet::on_exit(AppState::FleetEditor).with_system(screen_cleanup));
    }
}

#[derive(Component)]
struct RightSide;

#[derive(Component)]
struct GoButton;

fn display_ships(mut commands: Commands, asset_server: Res<AssetServer>, fleet: Res<Fleet>) {
    let n_columns = 3;
    let n_rows = 8;
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(Screen(AppState::FleetEditor))
        .with_children(|screen| {
            screen
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|left_side| {
                    for i in 0..n_columns {
                        left_side
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(33.3), Val::Percent(100.0)),
                                    justify_content: JustifyContent::FlexEnd,
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..default()
                                },
                                color: Color::NONE.into(),
                                ..default()
                            })
                            .with_children(|column| {
                                for j in 0..n_rows {
                                    let index = i * n_rows + j;
                                    column
                                        .spawn_bundle(ButtonBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(75.0), Val::Px(75.0)),
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
                                            visibility: Visibility {
                                                is_visible: index <= fleet.0.len(),
                                            },
                                            ..default()
                                        })
                                        .insert(ShipIndex(index))
                                        .with_children(|button| {
                                            if index < fleet.0.len() {
                                                let ship = &fleet.0[index];
                                                button.spawn_bundle(ImageBundle {
                                                    image: asset_server
                                                        .load(ship.parts.whole_ship)
                                                        .into(),
                                                    style: Style {
                                                        size: Size::new(Val::Auto, Val::Px(50.0)),
                                                        ..default()
                                                    },
                                                    focus_policy: FocusPolicy::Pass,
                                                    ..default()
                                                });
                                                button.spawn_bundle(
                                                    TextBundle::from_section(
                                                        ship.strength.0.to_string(),
                                                        TextStyle {
                                                            font: asset_server
                                                                .load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                                                            font_size: 15.0,
                                                            color: Color::DARK_GRAY,
                                                        },
                                                    )
                                                    .with_text_alignment(
                                                        TextAlignment::BOTTOM_CENTER,
                                                    ),
                                                );
                                            } else if index == fleet.0.len() {
                                                button.spawn_bundle(
                                                    TextBundle::from_section(
                                                        "+",
                                                        TextStyle {
                                                            font: asset_server
                                                                .load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                                                            font_size: 50.0,
                                                            color: Color::DARK_GRAY,
                                                        },
                                                    )
                                                    .with_text_alignment(TextAlignment::CENTER),
                                                );
                                            }
                                        });
                                }
                            });
                    }
                });
            screen
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(70.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|right_side| {
                    right_side
                        .spawn_bundle(
                            TextBundle::default().with_text_alignment(TextAlignment::CENTER_LEFT),
                        )
                        .insert(RightSide);
                    right_side
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(75.0), Val::Px(75.0)),
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
                        .insert(GoButton)
                        .with_children(|button| {
                            button.spawn_bundle(
                                TextBundle::from_section(
                                    "Go!",
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

fn activate_ships(
    mut state: ResMut<State<AppState>>,
    mut button_query: Query<
        (&Interaction, &ShipIndex, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut fleet: ResMut<Fleet>,
) {
    for (interaction, ship_index, mut color) in button_query.iter_mut() {
        let active = if ship_index.0 < fleet.0.len() {
            fleet.0[ship_index.0].active
        } else {
            false
        };
        *color = if active {
            match *interaction {
                Interaction::Hovered => Color::DARK_GREEN.into(),
                Interaction::None => Color::GREEN.into(),
                Interaction::Clicked => {
                    fleet.0[ship_index.0].active = false;
                    Color::ALICE_BLUE.into()
                }
            }
        } else {
            match *interaction {
                Interaction::Hovered => Color::GRAY.into(),
                Interaction::None => Color::ALICE_BLUE.into(),
                Interaction::Clicked => {
                    if ship_index.0 < fleet.0.len() {
                        fleet.0[ship_index.0].active = true;
                    } else {
                        state.set(AppState::ShipEditor).unwrap();
                    };
                    Color::GREEN.into()
                }
            }
        }
    }
}

fn go_button_interaction(
    mut state: ResMut<State<AppState>>,
    mut button_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>, With<GoButton>),
    >,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::GRAY.into(),
            Interaction::None => Color::ALICE_BLUE.into(),
            Interaction::Clicked => {
                state.set(AppState::Battle).unwrap();
                Color::GREEN.into()
            }
        }
    }
}

fn go_button_activation(
    mut button_query: Query<&mut Visibility, (With<Button>, With<GoButton>)>,
    fleet: ResMut<Fleet>,
) {
    for mut visibility in button_query.iter_mut() {
        *visibility = Visibility {
            is_visible: fleet.0.iter().any(|ship| ship.active),
        }
    }
}

fn update_strength(
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Text, With<RightSide>>,
    fleet: Res<Fleet>,
) {
    let mut text = query.single_mut();
    *text = Text::from_sections([TextSection::new(
        format!(
            "Fleet Information\nStrength: {}\nCombinations: {}\n",
            fleet.strength(),
            fleet.combination_bonus()
        ),
        TextStyle {
            font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
            font_size: 60.0,
            color: Color::GRAY,
        },
    )]);
}