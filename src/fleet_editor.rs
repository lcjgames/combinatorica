use bevy::prelude::*;
use bevy::render::render_resource::BindingType::Texture;

use crate::state::*;

pub struct FleetEditor;

impl Plugin for FleetEditor {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(AppState::FleetEditor).with_system(display_ships))
            // .add_system_set(SystemSet::on_update(AppState::FleetEditor).with_system(activate_ships))
            .add_system_set(SystemSet::on_exit(AppState::FleetEditor).with_system(screen_cleanup));
    }
}

// TODO: move this out of here
struct Parts {
    pub whole_ship: Handle<Image>, // TODO: divide this into parts
}

// TODO: move this out of here
struct Strength(i32);

// TODO: move this out of here
struct Ship {
    pub parts: Parts,
    pub strength: Strength,
}

fn display_ships(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ships = vec![
        Ship {
            parts: Parts {
                whole_ship: asset_server.load("spaceshooter/PNG/playerShip1_blue.png"), //TODO: move loading to loading state
            },
            strength: Strength(42),
        },
        Ship {
            parts: Parts {
                whole_ship: asset_server.load("spaceshooter/PNG/playerShip2_green.png"), //TODO: move loading to loading state
            },
            strength: Strength(78),
        },
        Ship {
            parts: Parts {
                whole_ship: asset_server.load("spaceshooter/PNG/playerShip3_orange.png"), //TODO: move loading to loading state
            },
            strength: Strength(55),
        },
    ]; // TODO: this is hardcoded just for now
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(20.0), Val::Percent(100.0)),
            justify_content: JustifyContent::FlexEnd,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    }).with_children(|node| {
        for ship in ships {
            node
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(75.0), Val::Px(75.0)),
                        margin: UiRect::new(Val::Percent(10.0), Val::Auto, Val::Auto, Val::Percent(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..default()
                    },
                    color: Color::ALICE_BLUE.into(),
                    ..default()
                })
                .with_children(|button| {
                    button.spawn_bundle(ImageBundle {
                        image: ship.parts.whole_ship.into(),
                        style: Style {
                            size: Size::new(Val::Auto, Val::Px(50.0)),
                            ..default()
                        },
                        ..default()
                    });
                    button.spawn_bundle(TextBundle::from_section(
                        ship.strength.0.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
                            font_size: 15.0,
                            color: Color::DARK_GRAY,
                        },
                    ).with_text_alignment(TextAlignment::BOTTOM_CENTER));
                })
                .insert(Screen(AppState::MainMenu));
        }
    });
}
