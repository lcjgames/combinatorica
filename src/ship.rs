use bevy::prelude::*;

use std::ops::Add;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Fleet>();
    }
}

#[derive(Component)]
pub struct ShipIndex(pub usize);

pub struct Parts {
    pub whole_ship: &'static str, // TODO: divide this into parts
}

pub struct Strength(pub i32);

pub struct Ship {
    pub parts: Parts,
    pub strength: Strength,
    pub active: bool,
}

pub struct Fleet(pub Vec<Ship>);

impl Default for Fleet {
    fn default() -> Self {
        Fleet(vec![
            Ship {
                parts: Parts {
                    whole_ship: "spaceshooter/PNG/playerShip1_blue.png", // TODO: use consts
                },
                strength: Strength(42),
                active: false,
            },
            Ship {
                parts: Parts {
                    whole_ship: "spaceshooter/PNG/playerShip2_green.png",
                },
                strength: Strength(78),
                active: false,
            },
            Ship {
                parts: Parts {
                    whole_ship: "spaceshooter/PNG/playerShip3_orange.png",
                },
                strength: Strength(55),
                active: false,
            },
        ])
    }
}

impl Fleet {
    pub fn strength(&self) -> i32 {
        self.0
            .iter()
            .filter(|ship| ship.active)
            .map(|ship| ship.strength.0)
            .sum()
    }
}
