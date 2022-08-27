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
            wings: vec![Wings {
                style: WingsStyle::TYPE3,
                color: PartColor::RED,
            }],
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

impl OwnedParts {
    pub fn len(&self, part_type: PartType) -> usize {
        match part_type {
            PartType::Cockpit => self.cockpit.len(),
            PartType::Engine => self.engine.len(),
            PartType::Wings => self.wings.len(),
            PartType::Lasergun => self.lasergun.len(),
        }
    }

    pub fn get_image(&self, part_type: PartType, index: usize) -> String {
        let type_name = match part_type {
            PartType::Cockpit => "cockpit",
            PartType::Engine => "engine",
            PartType::Wings => "wing",
            PartType::Lasergun => "gun",
        };
        let color = match part_type {
            PartType::Cockpit => self.cockpit[index].color.name(),
            PartType::Wings => self.wings[index].color.name(),
            PartType::Engine | PartType::Lasergun => "",
        };
        let separator = match part_type {
            PartType::Cockpit | PartType::Wings => "_",
            PartType::Engine | PartType::Lasergun => "",
        };
        let style = match part_type {
            PartType::Cockpit => self.cockpit[index].style.number().to_string(),
            PartType::Engine => self.engine[index].style.number().to_string(),
            PartType::Wings => self.wings[index].style.number().to_string(),
            PartType::Lasergun => format!("{:02}", self.lasergun[index].style.number()),
        };
        format!(
            "spaceshooter/PNG/Parts/{}{}{}{}.png",
            type_name, color, separator, style
        )
    }

    pub fn get_description(&self, part_type: PartType, index: usize) -> String {
        //TODO
        "Description".to_string()
    }

    pub fn at_least_one_each(&self) -> bool {
        self.cockpit.len() > 0
            && self.engine.len() > 0
            && self.wings.len() > 0
            && self.lasergun.len() > 0
    }
}

#[derive(Default)]
pub struct BuildingShip {
    pub cockpit_index: usize,
    pub engine_index: usize,
    pub wings_index: usize,
    pub lasergun_index: usize,
}

impl BuildingShip {
    pub fn set(&mut self, part_type: PartType, index: usize) {
        match part_type {
            PartType::Cockpit => self.cockpit_index = index,
            PartType::Engine => self.engine_index = index,
            PartType::Wings => self.wings_index = index,
            PartType::Lasergun => self.lasergun_index = index,
        }
    }
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

impl PartColor {
    fn name(&self) -> &'static str {
        match self {
            PartColor::BLUE => "Blue",
            PartColor::GREEN => "Green",
            PartColor::RED => "Red",
            PartColor::YELLOW => "Yellow",
        }
    }
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

impl CockpitStyle {
    fn number(&self) -> usize {
        match self {
            CockpitStyle::TYPE0 => 0,
            CockpitStyle::TYPE1 => 1,
            CockpitStyle::TYPE2 => 2,
            CockpitStyle::TYPE3 => 3,
            CockpitStyle::TYPE4 => 4,
            CockpitStyle::TYPE5 => 5,
            CockpitStyle::TYPE6 => 6,
            CockpitStyle::TYPE7 => 7,
        }
    }
}

enum EngineStyle {
    TYPE1,
    TYPE2,
    TYPE3,
    TYPE4,
    TYPE5,
}

impl EngineStyle {
    fn number(&self) -> usize {
        match self {
            EngineStyle::TYPE1 => 1,
            EngineStyle::TYPE2 => 2,
            EngineStyle::TYPE3 => 3,
            EngineStyle::TYPE4 => 4,
            EngineStyle::TYPE5 => 5,
        }
    }
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

impl WingsStyle {
    fn number(&self) -> usize {
        match self {
            WingsStyle::TYPE0 => 0,
            WingsStyle::TYPE1 => 1,
            WingsStyle::TYPE2 => 2,
            WingsStyle::TYPE3 => 3,
            WingsStyle::TYPE4 => 4,
            WingsStyle::TYPE5 => 5,
            WingsStyle::TYPE6 => 6,
            WingsStyle::TYPE7 => 7,
        }
    }
}

enum LaserGunStyle {
    TYPE0,
    TYPE1,
    TYPE2,
    TYPE3,
    TYPE4,
    TYPE5,
    TYPE6,
    TYPE7,
    TYPE8,
    TYPE9,
    TYPE10,
}

impl LaserGunStyle {
    fn number(&self) -> usize {
        match self {
            LaserGunStyle::TYPE0 => 0,
            LaserGunStyle::TYPE1 => 1,
            LaserGunStyle::TYPE2 => 2,
            LaserGunStyle::TYPE3 => 3,
            LaserGunStyle::TYPE4 => 4,
            LaserGunStyle::TYPE5 => 5,
            LaserGunStyle::TYPE6 => 6,
            LaserGunStyle::TYPE7 => 7,
            LaserGunStyle::TYPE8 => 8,
            LaserGunStyle::TYPE9 => 9,
            LaserGunStyle::TYPE10 => 10,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum PartType {
    #[default]
    Cockpit,
    Engine,
    Wings,
    Lasergun,
}

#[derive(Clone, Default)]
pub struct PartSelectionEvent(pub PartType);
