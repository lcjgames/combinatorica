use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_main_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}
