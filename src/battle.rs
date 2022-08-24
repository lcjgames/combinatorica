use bevy::prelude::*;

use crate::ship::*;
use crate::state::*;

pub struct Battle;

impl Plugin for Battle {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn_ships))
            .add_system_set(SystemSet::on_exit(AppState::Battle).with_system(screen_cleanup));
    }
}

fn spawn_ships(mut commands: Commands, asset_server: Res<AssetServer>, fleet: Res<Fleet>) {
    for (index, ship) in fleet.0.iter().enumerate().filter(|(_, ship)| ship.active) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(ship.parts.whole_ship),
                ..default()
            })
            .insert(ShipIndex(index));
    }
}
