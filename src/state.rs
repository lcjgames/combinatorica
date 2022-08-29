use bevy::prelude::*;
use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    FleetEditor,
    PartSelection,
    ShipEditor,
    Battle,
    Shop,
}

#[derive(Component)]
pub struct Screen(pub AppState);

pub fn screen_cleanup(
    state: Res<State<AppState>>,
    mut commands: Commands,
    query: Query<(Entity, &Screen)>,
) {
    for (id, screen) in query.iter() {
        if &screen.0 == state.current() {
            commands.entity(id).despawn_recursive();
        }
    }
}
