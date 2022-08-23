use bevy::prelude::*;
use bevy::render::render_resource::BindingType::Texture;

use crate::ship::*;
use crate::state::*;

pub struct FleetEditor;

impl Plugin for FleetEditor {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::FleetEditor).with_system(display_ships))
            .add_system_set(SystemSet::on_update(AppState::FleetEditor).with_system(activate_ships))
            .add_system_set(SystemSet::on_exit(AppState::FleetEditor).with_system(screen_cleanup));
    }
}

fn display_ships(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    active_fleet: Res<ActiveFleet>,
    inactive_fleet: Res<InactiveFleet>,
) {
    let n_rows = 8;
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexEnd,
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|node| {
            for row in 0..n_rows {
                node.spawn_bundle(ButtonBundle {
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
                .with_children(|button| {
                    if row < inactive_fleet.0.len() {
                        let ship = &inactive_fleet.0[row];
                        button.spawn_bundle(ImageBundle {
                            image: asset_server.load(ship.parts.whole_ship).into(),
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(50.0)),
                                ..default()
                            },
                            ..default()
                        });
                        button.spawn_bundle(
                            TextBundle::from_section(
                                ship.strength.0.to_string(),
                                TextStyle {
                                    font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                                    font_size: 15.0,
                                    color: Color::DARK_GRAY,
                                },
                            )
                            .with_text_alignment(TextAlignment::BOTTOM_CENTER),
                        );
                    } else {
                        button.spawn_bundle(
                            TextBundle::from_section(
                                "+",
                                TextStyle {
                                    font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                                    font_size: 50.0,
                                    color: Color::DARK_GRAY,
                                },
                            )
                            .with_text_alignment(TextAlignment::CENTER),
                        );
                    }
                });
            }
        })
        .insert(Screen(AppState::MainMenu));
}

pub fn activate_ships(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        *color = match *interaction {
            Interaction::Hovered => Color::GRAY.into(),
            Interaction::None => Color::ALICE_BLUE.into(),
            Interaction::Clicked => {
                //TODO
                Color::ALICE_BLUE.into()
            }
        }
    }
}
