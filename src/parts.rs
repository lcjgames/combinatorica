use bevy::prelude::*;

pub struct Parts;

impl Plugin for Parts {
    fn build(&self, app: &mut App) {
        app.init_resource::<OwnedParts>()
            .init_resource::<BuildingShip>();
    }
}

//TODO: #[derive(Default)]
#[derive(Clone)]
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

    pub fn add_random(&mut self) {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let part_type = PartType::from(rng.gen_range(0..4));
        match part_type {
            PartType::Cockpit => {
                let style = CockpitStyle::from(rng.gen_range(0..=7));
                let color = PartColor::from(rng.gen_range(0..4));
                crate::log::console_log!("New part: {:?} {:?} {:?}", color, style, part_type);
                self.cockpit.push(Cockpit { style, color });
            }
            PartType::Engine => {
                let style = EngineStyle::from(rng.gen_range(1..=5));
                crate::log::console_log!("New part: {:?} {:?}", style, part_type);
                self.engine.push(Engine { style });
            }
            PartType::Wings => {
                let style = WingsStyle::from(rng.gen_range(0..=7));
                let color = PartColor::from(rng.gen_range(0..4));
                crate::log::console_log!("New part: {:?} {:?} {:?}", color, style, part_type);
                self.wings.push(Wings { style, color });
            }
            PartType::Lasergun => {
                let style = LaserGunStyle::from(rng.gen_range(0..=10));
                crate::log::console_log!("New part: {:?} {:?}", style, part_type);
                self.lasergun.push(LaserGun { style });
            }
        }
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

#[derive(Clone)]
pub struct Cockpit {
    style: CockpitStyle,
    color: PartColor,
    //TODO: bonuses
}

#[derive(Clone)]
pub struct Engine {
    style: EngineStyle,
    //TODO: bonuses
}

#[derive(Clone)]
pub struct Wings {
    style: WingsStyle,
    color: PartColor,
    //TODO: bonuses
}

#[derive(Clone)]
pub struct LaserGun {
    style: LaserGunStyle,
    //TODO: bonuses
}

#[derive(Clone, Debug)]
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

impl From<i32> for PartColor {
    fn from(n: i32) -> Self {
        match n {
            0 => PartColor::BLUE,
            1 => PartColor::GREEN,
            2 => PartColor::RED,
            3 => PartColor::YELLOW,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
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

impl From<i32> for CockpitStyle {
    fn from(n: i32) -> Self {
        match n {
            0 => CockpitStyle::TYPE0,
            1 => CockpitStyle::TYPE1,
            2 => CockpitStyle::TYPE2,
            3 => CockpitStyle::TYPE3,
            4 => CockpitStyle::TYPE4,
            5 => CockpitStyle::TYPE5,
            6 => CockpitStyle::TYPE6,
            7 => CockpitStyle::TYPE7,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
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

impl From<i32> for EngineStyle {
    fn from(n: i32) -> Self {
        match n {
            1 => EngineStyle::TYPE1,
            2 => EngineStyle::TYPE2,
            3 => EngineStyle::TYPE3,
            4 => EngineStyle::TYPE4,
            5 => EngineStyle::TYPE5,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
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

impl From<i32> for WingsStyle {
    fn from(n: i32) -> Self {
        match n {
            0 => WingsStyle::TYPE0,
            1 => WingsStyle::TYPE1,
            2 => WingsStyle::TYPE2,
            3 => WingsStyle::TYPE3,
            4 => WingsStyle::TYPE4,
            5 => WingsStyle::TYPE5,
            6 => WingsStyle::TYPE6,
            7 => WingsStyle::TYPE7,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
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

impl From<i32> for LaserGunStyle {
    fn from(n: i32) -> Self {
        match n {
            0 => LaserGunStyle::TYPE0,
            1 => LaserGunStyle::TYPE1,
            2 => LaserGunStyle::TYPE2,
            3 => LaserGunStyle::TYPE3,
            4 => LaserGunStyle::TYPE4,
            5 => LaserGunStyle::TYPE5,
            6 => LaserGunStyle::TYPE6,
            7 => LaserGunStyle::TYPE7,
            8 => LaserGunStyle::TYPE8,
            9 => LaserGunStyle::TYPE9,
            10 => LaserGunStyle::TYPE10,
            _ => unreachable!(),
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

impl From<i32> for PartType {
    fn from(n: i32) -> Self {
        match n {
            0 => PartType::Cockpit,
            1 => PartType::Engine,
            2 => PartType::Wings,
            3 => PartType::Lasergun,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Default)]
pub struct PartSelectionEvent(pub PartType);
