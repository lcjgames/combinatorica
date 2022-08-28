use bevy::prelude::*;

use crate::state::*;

pub struct PilotLogPlugin;

impl Plugin for PilotLogPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PilotLogEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn_pilot_log))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(update_pilot_log));
    }
}

pub struct PilotLogEvent(String);

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

fn update_pilot_log(
    asset_server: Res<AssetServer>,
    mut text_query: Query<&mut Text, With<PilotLog>>,
) {
    let font = asset_server.load("fonts/Kenney Future.ttf"); //TODO: move loading to loading state;

    let mut text = text_query.single_mut();
    let mut logs = text.clone().sections;
    logs.push(TextSection::new("Hello\n", TextStyle::default()));
    let mut iterator = logs.iter().cloned();
    if logs.len() > 5 {
        iterator.advance_by(logs.len() - 5).unwrap();
    }
    *text = Text::from_sections(iterator.enumerate().map(|(index, section)| {
        TextSection::new(
            section.value,
            TextStyle {
                font: font.clone(),
                font_size: 40.0,
                color: Color::rgba(0.5, 0.5, 0.5, (index + 1) as f32 * 0.2),
            },
        )
    }));
}
