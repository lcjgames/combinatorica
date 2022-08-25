use bevy::prelude::*;
use rand::Rng;

use crate::ship::*;
use crate::state::*;

pub struct Battle;

impl Plugin for Battle {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn_target))
            .add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn_ships))
            .add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn_meteor_spawner))
            .add_system_set(SystemSet::on_enter(AppState::Battle).with_system(spawn_exit_timer))
            .add_system_set(
                SystemSet::on_enter(AppState::Battle).with_system(spawn_that_text_on_the_screen),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Battle).with_system(update_that_text_on_the_screen),
            )
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(update_target))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(target_force))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(ship_forces))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(steer_ships))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(spawn_meteors))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(movement))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(spawn_laser))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(despawn_laser))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(destroy_ships))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(exit_timer))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(exit_buttons))
            .add_system_set(SystemSet::on_update(AppState::Battle).with_system(show_all))
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
    ship_interaction_force: Vec3,
}

impl ShipForce {
    fn resultant(&self) -> Vec3 {
        let mut resultant = self.target_attraction_force + self.ship_interaction_force;
        resultant.z = 0.0;
        resultant
    }
}

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component)]
struct Meteor;

#[derive(Component)]
struct MeteorSpawner {
    timer: Timer,
    spawn_radius: f32,
    target_radius: f32,
}

#[derive(Component)]
struct HitBox {
    radius: f32,
}

const SHIP_HITBOX_SIZE: f32 = 5.0;
const METEOR_HITBOX_SIZE: f32 = 50.0;

#[derive(Component)]
struct Laser {
    timer: Timer,
}

#[derive(Component)]
struct ExitTimer(Timer);

#[derive(Component)]
struct ThatTextOnTheScreen;

fn spawn_target(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("spaceshooter/PNG/UI/cursor.png"), // TODO: move to loading
            transform: Transform::from_translation(Vec3::Z),
            ..default()
        })
        .insert(ShipTarget)
        .insert(Screen(AppState::Battle));
}

fn spawn_ships(mut commands: Commands, asset_server: Res<AssetServer>, fleet: Res<Fleet>) {
    for (index, ship) in fleet.0.iter().enumerate().filter(|(_, ship)| ship.active) {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(ship.parts.whole_ship),
                transform: Transform::from_translation(Vec3::new((20 * index) as f32, 0.0, 1.0))
                    .with_scale(Vec3::splat(0.3)),
                ..default()
            })
            .insert(ShipMarker)
            .insert(ShipIndex(index))
            .insert(ShipForce::default())
            .insert(Velocity::default())
            .insert(HitBox {
                radius: SHIP_HITBOX_SIZE,
            })
            .insert(Screen(AppState::Battle))
            .with_children(|meteor| {
                meteor.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.2, 0.2, 0.6, 0.3),
                        custom_size: Some(Vec2::splat(SHIP_HITBOX_SIZE * 1.414)),
                        ..default()
                    },
                    transform: Transform::from_translation(1.0 * Vec3::Z),
                    visibility: Visibility { is_visible: false },
                    ..default()
                });
            });
    }
}
fn spawn_meteor_spawner(mut commands: Commands) {
    commands
        .spawn()
        .insert(MeteorSpawner {
            timer: Timer::from_seconds(2.0, true),
            spawn_radius: 1000.0,
            target_radius: 500.0,
        })
        .insert(Screen(AppState::Battle));
}

fn spawn_exit_timer(mut commands: Commands) {
    commands
        .spawn()
        .insert(ExitTimer(Timer::from_seconds(60.0, false)));
}

fn spawn_that_text_on_the_screen(mut commands: Commands) {
    commands
        .spawn_bundle(TextBundle::default().with_text_alignment(TextAlignment::CENTER_LEFT))
        .insert(ThatTextOnTheScreen);
}

fn update_that_text_on_the_screen(
    asset_server: Res<AssetServer>,
    mut text_query: Query<&mut Text, With<ThatTextOnTheScreen>>,
    timer_query: Query<&ExitTimer>,
) {
    let mut text = text_query.single_mut();
    let timer = timer_query.single();
    *text = Text::from_sections([TextSection::new(
        format!(
            "Time left: {:.2}",
            (timer.0.duration() - timer.0.elapsed()).as_secs_f32()
        ),
        TextStyle {
            font: asset_server.load("fonts/Kenney Future.ttf"), //TODO: move loading to loading state
            font_size: 60.0,
            color: Color::GRAY,
        },
    )]);
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
            10.0 * direction
        };
    }
}

fn ship_forces(mut query: Query<(&Transform, &mut ShipForce), With<ShipMarker>>) {
    for (_, mut force) in query.iter_mut() {
        force.ship_interaction_force = Vec3::ZERO;
    }
    let mut iter = query.iter_combinations_mut();
    while let Some([(transform_1, mut force_1), (transform_2, mut force_2)]) = iter.fetch_next() {
        let separation_force = {
            let direction = transform_1.translation - transform_2.translation;
            let distance = direction.length();
            let factor = if distance > 200.0 {
                -100.0
            } else if distance > 50.0 {
                0.0
            } else if distance > 10.0 {
                1_000.0
            } else {
                10_000.0
            };
            factor * direction.normalize()
        };
        crate::log::console_log!("removing this log breaks everything");
        force_1.ship_interaction_force += separation_force;
        force_2.ship_interaction_force -= separation_force;
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
    }
}

fn spawn_laser(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ship_query: Query<&Transform, With<ShipMarker>>,
    mut meteor_query: Query<(&Transform, &HitBox), (With<Meteor>, Without<ShipMarker>)>,
) {
    for ship_transform in ship_query.iter() {
        for (meteor_transform, meteor_hitbox) in meteor_query.iter() {
            let distance_vector = ship_transform.translation - meteor_transform.translation;
            let distance = distance_vector.length();
            if distance < 100.0 + meteor_hitbox.radius {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: asset_server.load("spaceshooter/PNG/Lasers/laserRed05.png"),
                        transform: Transform::from_translation(
                            (ship_transform.translation + meteor_transform.translation) / 2.0
                                - 0.3 * Vec3::Z,
                        )
                        .with_rotation(Quat::from_rotation_arc(
                            Vec3::Y,
                            distance_vector.normalize(),
                        ))
                        .with_scale(Vec3::new(1.0, distance / 50.0, 1.0)),
                        ..default()
                    })
                    .insert(Laser {
                        timer: Timer::from_seconds(0.1, false),
                    })
                    .insert(Screen(AppState::Battle));
            }
        }
    }
}

fn despawn_laser(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Laser)>) {
    for (entity, mut laser) in query.iter_mut() {
        laser.timer.tick(time.delta());
        if laser.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_meteors(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut query: Query<&mut MeteorSpawner>,
) {
    use rand::prelude::*;
    use std::f32::consts::PI;

    let mut rng = thread_rng();
    let mut spawner = query.single_mut();

    let spawn_position = Vec3::from((
        spawner.spawn_radius * Vec2::from_angle(rng.gen_range(0.0..2.0 * PI)),
        0.5,
    ));
    let target_position = Vec3::from((
        spawner.target_radius * Vec2::from_angle(rng.gen_range(0.0..2.0 * PI)),
        0.5,
    ));
    let velocity = (target_position - spawn_position).normalize() * rng.gen_range(10.0..100.0);
    let size = rng.gen_range(0.3..1.5);

    spawner.timer.tick(time.delta());
    if spawner.timer.finished() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("spaceshooter/PNG/Meteors/meteorBrown_big1.png"),
                transform: Transform::from_translation(spawn_position)
                    .with_scale(Vec3::splat(size)),
                ..default()
            })
            .insert(Meteor)
            .insert(Velocity(velocity))
            .insert(HitBox {
                radius: size * METEOR_HITBOX_SIZE,
            })
            .insert(Screen(AppState::Battle))
            .with_children(|meteor| {
                meteor.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.2, 0.2, 0.6, 0.3),
                        custom_size: Some(Vec2::splat(METEOR_HITBOX_SIZE * 1.414)),
                        ..default()
                    },
                    transform: Transform::from_translation(1.0 * Vec3::Z),
                    visibility: Visibility { is_visible: false },
                    ..default()
                });
            });
    }
}

fn movement(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

fn destroy_ships(
    mut commands: Commands,
    mut ship_query: Query<(Entity, &Transform, &HitBox), With<ShipMarker>>,
    mut meteor_query: Query<(&Transform, &HitBox), (With<Meteor>, Without<ShipMarker>)>,
) {
    for (ship_entity, ship_transform, ship_hitbox) in ship_query.iter() {
        for (meteor_transform, meteor_hitbox) in meteor_query.iter() {
            let distance = (ship_transform.translation - meteor_transform.translation).length();
            if distance < ship_hitbox.radius + meteor_hitbox.radius {
                commands.entity(ship_entity).despawn_recursive();
            }
        }
    }
}

fn exit_timer(
    time: Res<Time>,
    mut query: Query<&mut ExitTimer>,
    mut state: ResMut<State<AppState>>,
) {
    let mut timer = query.single_mut();
    timer.0.tick(time.delta());
    if timer.0.finished() {
        state.set(AppState::FleetEditor).unwrap();
    }
}

fn exit_buttons(input: Res<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if input.just_pressed(KeyCode::Escape) || input.just_pressed(KeyCode::Q) {
        state.set(AppState::FleetEditor).unwrap();
    }
}

fn show_all(input: Res<Input<KeyCode>>, mut query: Query<&mut Visibility>) {
    if input.pressed(KeyCode::H) && input.pressed(KeyCode::B) {
        for mut visibility in query.iter_mut() {
            visibility.is_visible = true;
        }
    }
}
