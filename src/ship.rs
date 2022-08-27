use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Fleet>();
    }
}

#[derive(Component)]
pub struct ShipIndex(pub usize);

#[derive(Clone)]
pub struct Parts {
    pub whole_ship: &'static str, // TODO: divide this into parts
}

#[derive(Clone)]
pub struct Strength(pub f32);

#[derive(Clone)]
pub struct Ship {
    pub parts: Parts,
    pub strength: Strength,
    pub active: bool,
    pub destroyed: bool,
}

pub struct Fleet(pub Vec<Ship>);

impl Default for Fleet {
    fn default() -> Self {
        Fleet(vec![
            Ship {
                parts: Parts {
                    whole_ship: "spaceshooter/PNG/playerShip1_blue.png", // TODO: use consts
                },
                strength: Strength(42.0),
                active: false,
                destroyed: false,
            },
            Ship {
                parts: Parts {
                    whole_ship: "spaceshooter/PNG/playerShip2_green.png",
                },
                strength: Strength(78.1),
                active: false,
                destroyed: false,
            },
            Ship {
                parts: Parts {
                    whole_ship: "spaceshooter/PNG/playerShip3_orange.png",
                },
                strength: Strength(55.3),
                active: false,
                destroyed: false,
            },
        ])
    }
}

impl Fleet {
    pub fn strength(&self) -> f32 {
        self.0
            .iter()
            .filter(|ship| ship.active)
            .map(|ship| ship.strength.0)
            .sum()
    }

    pub fn combination_bonus(&self) -> f32 {
        use crate::combinatorics::combination;

        let total = self.0.len();
        let active = self.0.iter().filter(|ship| ship.active).count();
        combination(total, active) as f32
    }
}
