use bevy::prelude::*;
use bevy::render::render_resource::BindingType::Texture;
use bevy::ui::FocusPolicy;

use crate::ship::*;
use crate::state::*;

pub struct FleetEditor;

impl Plugin for FleetEditor {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::FleetEditor).with_system(display_ships))
            .add_system_set(SystemSet::on_update(AppState::FleetEditor).with_system(activate_ships))
            .add_system_set(
                SystemSet::on_update(AppState::FleetEditor).with_system(update_strength),
            )
            .add_system_set(SystemSet::on_exit(AppState::FleetEditor).with_system(screen_cleanup));
    }
}

#[derive(Component)]
struct StrengthText;

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
                                            } else {
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
                        .insert(StrengthText);
                });
        })
        .insert(Screen(AppState::MainMenu));
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

fn update_strength(
    asset_server: Res<AssetServer>,
    mut query: Query<&mut Text, With<StrengthText>>,
    fleet: Res<Fleet>,
) {
    for mut text in query.iter_mut() {
        *text = Text::from_section(
            format!("Strength: {}", fleet.strength()),
            TextStyle {
                font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                font_size: 75.0,
                color: Color::GRAY,
            },
        );
    }
}
