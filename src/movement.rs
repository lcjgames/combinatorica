use bevy::prelude::*;

#[derive(Component, Default, Deref, DerefMut)]
pub struct Velocity(Vec3);

pub fn movement(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0;
    }
}
