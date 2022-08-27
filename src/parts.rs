use bevy::prelude::*;

pub struct Parts;

impl Plugin for Parts {
    fn build(&self, app: &mut App) {
        app.init_resource::<OwnedParts>()
            .init_resource::<BuildingShip>();
    }
}

//TODO: #[derive(Default)]
pub struct OwnedParts {
    pub cockpit: Vec<Cockpit>,
    pub engine: Vec<Engine>,
    pub wings: Vec<Wings>,
    pub lasergun: Vec<LaserGun>,
}

impl Default for OwnedParts {
    fn default() -> Self {
        Self {
            cockpit: vec![Cockpit {
                style: CockpitStyle::TYPE3,
                color: PartColor::BLUE,
            }],
            engine: vec![
                Engine {
                    style: EngineStyle::TYPE2,
                },
                Engine {
                    style: EngineStyle::TYPE1,
                },
                Engine {
                    style: EngineStyle::TYPE5,
                },
            ],
            wings: Vec::new(),
            lasergun: vec![
                LaserGun {
                    style: LaserGunStyle::TYPE0,
                },
                LaserGun {
                    style: LaserGunStyle::TYPE0,
                },
            ],
        }
    }
}

#[derive(Default)]
pub struct BuildingShip {
    cockpit: Option<Cockpit>,
    engine: Option<Engine>,
    wings: Option<Wings>,
    lasergun: Option<LaserGun>,
}

pub struct Cockpit {
    style: CockpitStyle,
    color: PartColor,
    //TODO: bonuses
}

pub struct Engine {
    style: EngineStyle,
    //TODO: bonuses
}

pub struct Wings {
    style: WingsStyle,
    color: PartColor,
    //TODO: bonuses
}

pub struct LaserGun {
    style: LaserGunStyle,
    //TODO: bonuses
}

enum PartColor {
    BLUE,
    GREEN,
    RED,
    YELLOW,
}

enum CockpitStyle {
    TYPE0,
    TYPE1,
    TYPE2,
    TYPE3,
    TYPE4,
    TYPE5,
    TYPE6,
    TYPE7,
}

enum EngineStyle {
    TYPE1,
    TYPE2,
    TYPE3,
    TYPE4,
    TYPE5,
}

enum WingsStyle {
    TYPE0,
    TYPE1,
    TYPE2,
    TYPE3,
    TYPE4,
    TYPE5,
    TYPE6,
    TYPE7,
}

enum LaserGunStyle {
    TYPE0,
    TYPE1,
    TYPE2,
    TYPE3,
    TYPE4,
    TYPE5,
    TYPE6,
    TYPE8,
    TYPE9,
    TYPE10,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum PartType {
    #[default]
    Cockpit,
    Engine,
    Wings,
    Lasergun,
}

#[derive(Default)]
pub struct PartSelectionEvent(pub PartType);
