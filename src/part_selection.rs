use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::parts::*;
use crate::state::*;

pub struct PartSelection;

impl Plugin for PartSelection {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::PartSelection).with_system(display_parts))
            .add_system_set(
                SystemSet::on_update(AppState::PartSelection).with_system(part_button_interaction),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::PartSelection).with_system(screen_cleanup),
            );
    }
}

#[derive(Component)]
struct PartIndex(PartType, usize);

#[derive(Component)]
struct DescriptionText;

fn display_parts(
    mut part_selection_event_reader: EventReader<PartSelectionEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    owned_parts: Res<OwnedParts>,
) {
    let part_type = part_selection_event_reader
        .iter()
        .next()
        .cloned()
        .unwrap_or_default()
        .0;
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
        .insert(Screen(AppState::PartSelection))
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
                            display_part_buttons(left_side, &asset_server, &owned_parts, part_type);
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
                        .with_children(|left_side| {
                            left_side
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        "", //TODO
                                        TextStyle {
                                            font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                                            font_size: 15.0,
                                            color: Color::DARK_GRAY,
                                        },
                                    )
                                    .with_text_alignment(TextAlignment::BOTTOM_CENTER),
                                )
                                .insert(DescriptionText);
                        });
                });
            screen.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            });
        });
}

fn display_part_buttons(
    node: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    owned_parts: &OwnedParts,
    part_type: PartType,
) {
    let n_columns = 3;
    let n_rows = 5;
    for i in 0..n_columns {
        node.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(33.3), Val::Percent(100.0)),
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
                            is_visible: index < owned_parts.len(part_type),
                        },
                        ..default()
                    })
                    .insert(PartIndex(part_type, index))
                    .with_children(|button| {
                        if index < owned_parts.len(part_type) {
                            button.spawn_bundle(ImageBundle {
                                image: asset_server
                                    .load(owned_parts.get_image(part_type, index).as_str())
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
                                    "part", //TODO
                                    TextStyle {
                                        font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                                        font_size: 15.0,
                                        color: Color::DARK_GRAY,
                                    },
                                )
                                .with_text_alignment(TextAlignment::BOTTOM_CENTER),
                            );
                        }
                    });
            }
        });
    }
}

fn part_button_interaction(
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
    mut button_query: Query<
        (&Interaction, &mut UiColor, &PartIndex),
        (Changed<Interaction>, With<Button>),
    >,
    owned_parts: Res<OwnedParts>,
    mut ship: ResMut<BuildingShip>,
    mut query: Query<&mut Text, With<DescriptionText>>,
) {
    for (interaction, mut color, part_index) in button_query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => {
                let mut text = query.single_mut();
                *text = Text::from_section(
                    format!(
                        "Description:\n{}",
                        owned_parts.get_description(part_index.0, part_index.1)
                    ),
                    TextStyle {
                        font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                        font_size: 60.0,
                        color: Color::GRAY,
                    },
                );
                Color::GRAY.into()
            }
            Interaction::None => Color::ALICE_BLUE.into(),
            Interaction::Clicked => {
                ship.set(part_index.0, part_index.1);
                state.set(AppState::ShipEditor).unwrap();
                Color::GREEN.into()
            }
        }
    }
}
