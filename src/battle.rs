use bevy::prelude::*;

use crate::ship::*;
use crate::state::*;

pub struct Battle;

impl Plugin for Battle {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn_target))
            .add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn_ships))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(update_target))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(target_force))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(boid_forces))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(steer_ships))
            .add_system_set(SystemSet::on_exit(AppState::Battle).with_system(screen_cleanup));
    }
}

#[derive(Component)]
struct ShipTarget;

#[derive(Component)]
struct ShipMarker;

#[derive(Component, Default)]
struct ShipForce {
    target_attraction_force: Vec3,
    separation_force: Vec3,
    cohesion_force: Vec3,
}

#[derive(Component, Default)]
struct Velocity(Vec3);

impl ShipForce {
    fn resultant(&self) -> Vec3 {
        self.target_attraction_force + self.separation_force + self.cohesion_force
    }
}

fn spawn_target(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("spaceshooter/PNG/UI/cursor.png"), // TODO: move to loading
            transform: Transform::from_translation(Vec3::Z),
            ..default()
        })
        .insert(ShipTarget);
}

fn spawn_ships(mut commands: Commands, asset_server: Res<AssetServer>, fleet: Res<Fleet>) {
    for (index, ship) in fleet.0.iter().enumerate().filter(|(_, ship)| ship.active) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(ship.parts.whole_ship),
                transform: Transform::from_translation(Vec3::new((10 * index) as f32, 0.0, 1.0))
                    .with_scale(Vec3::splat(0.3)),
                ..default()
            })
            .insert(ShipMarker)
            .insert(ShipIndex(index))
            .insert(ShipForce::default())
            .insert(Velocity::default())
            .insert(Screen(AppState::Battle));
    }
}

fn update_target(windows: Res<Windows>, mut query: Query<&mut Transform, With<ShipTarget>>) {
    let window = windows.get_primary().unwrap();
    if let Some(screen_position) = window.cursor_position() {
        let mut transform = query.single_mut();
        transform.translation = Vec3::new(
            screen_position.x - window.width() / 2.0,
            screen_position.y - window.height() / 2.0,
            2.0,
        );
    }
}

fn target_force(
    mut ship_query: Query<(&Transform, &mut ShipForce), With<ShipMarker>>,
    target_query: Query<&Transform, (Without<ShipMarker>, With<ShipTarget>)>,
) {
    let target_transform = target_query.single();
    for (ship_transform, mut force) in ship_query.iter_mut() {
        force.target_attraction_force = {
            let direction = target_transform.translation - ship_transform.translation;
            1_000.0 * direction.normalize()
        };
    }
}

fn boid_forces(mut query: Query<(&Transform, &mut ShipForce), With<ShipMarker>>) {
    for (_, mut force) in query.iter_mut() {
        force.separation_force = Vec3::ZERO;
        force.cohesion_force = Vec3::ZERO;
    }
    let mut iter = query.iter_combinations_mut();
    while let Some([(transform_1, mut force_1), (transform_2, mut force_2)]) = iter.fetch_next() {
        let separation_force = {
            let direction = transform_1.translation - transform_2.translation;
            let distance = direction.length();
            let factor = if distance > 200.0 {
                -100.0
            } else if distance > 100.0 {
                0.0
            } else if distance > 10.0 {
                100.0
            } else {
                1_000.0
            };
            factor * direction.normalize()
        };
        crate::log::console_log!("separation force: {}", separation_force); //removing this log breaks everything
        force_1.separation_force += separation_force;
        force_2.separation_force -= separation_force;
    }
}

fn steer_ships(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity, &ShipForce), With<ShipMarker>>,
) {
    let dt = time.delta_seconds();
    for (mut transform, mut velocity, force) in query.iter_mut() {
        let resultant_force = force.resultant();
        transform.rotation = Quat::from_rotation_arc(Vec3::Y, resultant_force.normalize());
        velocity.0 += resultant_force / 70.0 * dt;
        transform.translation += velocity.0 * dt;
    }
}
