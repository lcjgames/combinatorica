use bevy::prelude::*;

use crate::ship::*;
use crate::state::*;

pub struct PilotLogPlugin;

impl Plugin for PilotLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PilotLogEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn_pilot_log))
            .add_system_set(SystemSet::on_enter(AppState::Battle).with_system(hello))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(update_pilot_log));
    }
}

pub struct PilotLogEvent(pub String);

#[derive(Component)]
struct PilotLog;

fn spawn_pilot_log(mut commands: Commands) {
    commands
        .spawn_bundle(
            TextBundle::default()
                .with_text_alignment(TextAlignment::CENTER_LEFT)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        right: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                }),
        )
        .insert(PilotLog)
        .insert(Screen(AppState::Battle));
}

fn hello(mut event_writer: EventWriter<PilotLogEvent>, fleet: Res<Fleet>) {
    for ship in fleet.0.iter().filter(|ship| ship.active) {
        event_writer.send(PilotLogEvent(format!(
            "{} presenting for duties\n",
            ship.pilot_name
        )));
    }
}

fn update_pilot_log(
    asset_server: Res<AssetServer>,
    mut text_query: Query<&mut Text, With<PilotLog>>,
    mut event_reader: EventReader<PilotLogEvent>,
) {
    const MAX_MESSAGES: usize = 5;

    let font = asset_server.load("fonts/Kenney Future.ttf"); //TODO: move loading to loading state;

    let mut text = text_query.single_mut();
    let mut logs = text.clone().sections;
    for event in event_reader.iter() {
        logs.push(TextSection::new(event.0.clone(), TextStyle::default()));
    }
    let mut iterator = logs.iter().cloned();
    if logs.len() > MAX_MESSAGES {
        iterator.advance_by(logs.len() - MAX_MESSAGES).unwrap();
    }
    *text = Text::from_sections(iterator.enumerate().map(|(index, section)| {
        TextSection::new(
            section.value,
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: Color::rgba(0.5, 0.5, 0.5, (index + 1) as f32 * 0.2),
            },
        )
    }));
}
