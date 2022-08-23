use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InactiveFleet>()
            .init_resource::<ActiveFleet>();
    }
}

pub struct Parts {
    pub whole_ship: &'static str, // TODO: divide this into parts
}

pub struct Strength(pub i32);

pub struct Ship {
    pub parts: Parts,
    pub strength: Strength,
}

pub struct InactiveFleet(pub Vec<Ship>);

impl Default for InactiveFleet {
    fn default() -> Self {
        InactiveFleet(vec![
            Ship {
                parts: Parts {
                    whole_ship: "spaceshooter/PNG/playerShip1_blue.png", // TODO: use consts
                },
                strength: Strength(42),
            },
            Ship {
                parts: Parts {
                    whole_ship: "spaceshooter/PNG/playerShip2_green.png",
                },
                strength: Strength(78),
            },
            Ship {
                parts: Parts {
                    whole_ship: "spaceshooter/PNG/playerShip3_orange.png",
                },
                strength: Strength(55),
            },
        ])
    }
}

#[derive(Default)]
pub struct ActiveFleet(pub Vec<Ship>);
