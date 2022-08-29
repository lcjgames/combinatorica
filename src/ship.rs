use bevy::prelude::*;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Fleet>();
    }
}

#[derive(Component)]
pub struct ShipIndex(pub usize);

#[derive(Clone, Component)]
pub struct Strength(pub f32);

impl Strength {
    pub fn mine(&self) -> f32 {
        self.0 / 100.0
    }
}

#[derive(Clone)]
pub struct Ship {
    pub wings_sprite: String,
    pub cockpit_sprite: String,
    pub strength: Strength,
    pub active: bool,
    pub destroyed: bool,
    pub pilot_name: &'static str,
}

impl Ship {
    pub fn left_wing_position() -> Vec3 {
        Vec3::new(-30.0, 15.0, -0.1)
    }
    pub fn right_wing_position() -> Vec3 {
        let mut res = Self::left_wing_position();
        res.x = -res.x;
        res
    }
    pub fn cockpit_ui_style() -> Style {
        Style {
            size: Size::new(Val::Auto, Val::Percent(80.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(30.0),
                right: Val::Percent(30.0),
                top: Val::Percent(10.0),
                bottom: Val::Percent(10.0),
            },
            ..default()
        }
    }
    pub fn left_wing_ui_style() -> Style {
        Style {
            size: Size::new(Val::Auto, Val::Percent(80.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(60.0),
                right: Val::Percent(10.0),
                top: Val::Percent(10.0),
                bottom: Val::Percent(10.0),
            },
            ..default()
        }
    }
    pub fn right_wing_ui_style() -> Style {
        Style {
            size: Size::new(Val::Auto, Val::Percent(80.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Percent(10.0),
                right: Val::Percent(60.0),
                top: Val::Percent(10.0),
                bottom: Val::Percent(10.0),
            },
            ..default()
        }
    }
}

#[derive(Default)]
pub struct Fleet(pub Vec<Ship>);

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

    pub fn combination_bonus_relative(&self) -> f32 {
        use crate::combinatorics::max_combinations;

        self.combination_bonus() / max_combinations(self.0.len()) as f32
    }
}

pub const PILOT_NAMES: [&'static str; 25] = [
    "Serena",
    "Luiz",
    "Priscilla",
    "Jorge",
    "Antonio",
    "Solo",
    "Leela",
    "Niobe",
    "Cooper",
    "Fox",
    "Ayanami",
    "Shinji",
    "Aang",
    "Dumont",
    "Aldrin",
    "Willy",
    "Deckard",
    "Kirk",
    "Logan",
    "Parker",
    "OB1",
    "R2",
    "Stark",
    "X",
    "O'Hara",
];
