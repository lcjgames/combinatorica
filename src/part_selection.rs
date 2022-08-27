use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::parts::*;
use crate::state::*;

pub struct PartSelection;

impl Plugin for PartSelection {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::PartSelection).with_system(display_parts));
    }
}

#[derive(Component)]
struct PartIndex(usize);

fn display_parts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
                            display_part_buttons(left_side, &asset_server, &owned_parts);
                        });
                    upper_screen.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(70.0), Val::Percent(100.0)),
                            ..default()
                        },
                        color: Color::GREEN.into(),
                        ..default()
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
) {
    let n_columns = 3;
    let n_rows = 8;
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
                            is_visible: true, //TODO: for now
                        },
                        ..default()
                    })
                    .insert(PartIndex(index))
                    .with_children(|button| {
                        if index < owned_parts.cockpit.len() {
                            //TODO
                            let part = &owned_parts.cockpit[index]; //TODO
                            button.spawn_bundle(ImageBundle {
                                image: asset_server
                                    .load("spaceshooter/PNG/Parts/cockpitBlue_0.png")
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
