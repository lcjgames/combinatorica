use bevy::prelude::*;

pub struct Parts;

impl Plugin for Parts {
    fn build(&self, app: &mut App) {
        app.init_resource::<BuildingShip>();
    }
}

#[derive(Default)]
struct BuildingShip {
    cockpit: Option<Cockpit>,
    engine: Option<Engine>,
    wings: Option<Wings>,
    lasergun: Option<LaserGun>,
}

struct Cockpit {
    style: CockpitStyle,
    color: Color,
    //TODO: bonuses
}

struct Engine {
    style: EngineStyle,
    //TODO: bonuses
}

struct Wings {
    style: WingsStyle,
    color: Color,
    //TODO: bonuses
}

struct LaserGun {
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
