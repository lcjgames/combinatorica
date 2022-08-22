use bevy::prelude::*;

use crate::movement::Velocity;

#[derive(Component)]
pub struct Controls {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub jump: KeyCode,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            jump: KeyCode::Space,
        }
    }
}

pub fn input_handling(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Controls, &mut Velocity)>,
) {
    for (controls, mut velocity) in &mut query {
        if keyboard_input.pressed(controls.right) {
            velocity.x = 1.0
        } else if keyboard_input.pressed(controls.left) {
            velocity.x = -1.0
        } else {
            velocity.x = 0.0
        }
    }
}
