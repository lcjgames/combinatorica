use bevy::prelude::*;

use crate::state::*;

pub struct Shop;

impl Plugin for Shop {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Shop).with_system(placeholder));
    }
}

fn placeholder(mut state: ResMut<State<AppState>>) {
    state.set(AppState::FleetEditor).unwrap();
}
