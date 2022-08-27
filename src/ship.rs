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
pub struct Strength(pub f32);

#[derive(Clone)]
pub struct Ship {
    pub wings_sprite: String,
    pub cockpit_sprite: String,
    pub strength: Strength,
    pub active: bool,
    pub destroyed: bool,
}

pub struct Fleet(pub Vec<Ship>);

impl Default for Fleet {
    fn default() -> Self {
        Fleet(vec![Ship {
            wings_sprite: "spaceshooter/PNG/Parts/wingBlue_0.png".to_string(),
            cockpit_sprite: "spaceshooter/PNG/Parts/cockpitBlue_0.png".to_string(),
            strength: Strength(42.0),
            active: false,
            destroyed: false,
        }])
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
