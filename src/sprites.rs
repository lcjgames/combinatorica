use bevy::prelude::*;

#[derive(Default)]
pub struct SpriteSheets {
    pub player_fishy: Handle<TextureAtlas>,
    pub player_orcy: Handle<TextureAtlas>,
    pub player_pescy: Handle<TextureAtlas>,
    pub player_sharky: Handle<TextureAtlas>,
}

#[derive(Bundle)]
pub struct AnimatedSpriteBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_position: AnimatedSpriteInfo,
    pub animation_timer: AnimationTimer,
}

#[derive(Component)]
pub struct AnimatedSpriteInfo {
    line_number: usize,
    length: usize,
    repeating: bool,
}

impl AnimatedSpriteInfo {
    const N_COLUMNS: usize = 14;

    pub fn start(&self) -> usize {
        self.line_number * Self::N_COLUMNS
    }

    fn end(&self) -> usize {
        self.start() + self.length
    }

    fn next(&self, current_index: usize) -> usize {
        let next_index = current_index + 1;
        if next_index == self.end() {
            if self.repeating {
                self.start()
            } else {
                current_index
            }
        } else {
            next_index
        }
    }
}

pub const IDLE_FISH: AnimatedSpriteInfo = AnimatedSpriteInfo {
    line_number: 0,
    length: 14,
    repeating: true,
};
pub const RUNNING_FISH: AnimatedSpriteInfo = AnimatedSpriteInfo {
    line_number: 1,
    length: 6,
    repeating: true,
};
pub const JUMPING_FISH: AnimatedSpriteInfo = AnimatedSpriteInfo {
    line_number: 2,
    length: 1,
    repeating: true,
};
pub const FALLING_FISH: AnimatedSpriteInfo = AnimatedSpriteInfo {
    line_number: 3,
    length: 1,
    repeating: true,
};
pub const CROUCHING_FISH: AnimatedSpriteInfo = AnimatedSpriteInfo {
    line_number: 4,
    length: 1,
    repeating: true,
};
pub const DYING_BACKWARD_FISH: AnimatedSpriteInfo = AnimatedSpriteInfo {
    line_number: 5,
    length: 7,
    repeating: false,
};
pub const DYING_FORWARD_FISH: AnimatedSpriteInfo = AnimatedSpriteInfo {
    line_number: 6,
    length: 7,
    repeating: false,
};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &AnimatedSpriteInfo,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, sprite_position, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = sprite_position.next(sprite.index);
        }
    }
}

#[derive(Default)]
pub struct GroundTiles {
    pub metal: Handle<TextureAtlas>,
    pub rock: Handle<TextureAtlas>,
    pub wood: Handle<TextureAtlas>,
}

#[derive(Deref)]
pub struct TileIndex(usize);

impl TileIndex {
    pub const fn new(row: usize, column: usize) -> Self {
        Self(row * 17 + column)
    }
}

pub const TOP_LEFT_LEDGE: TileIndex = TileIndex::new(0, 0);
pub const TOP_MIDDLE_LEDGE: TileIndex = TileIndex::new(0, 1);
pub const TOP_RIGHT_LEDGE: TileIndex = TileIndex::new(0, 2);
// pub const EMPTY: TileIndex = TileIndex::new(0, 3);
pub const TOP_LEFTRIGHT_LEDGE: TileIndex = TileIndex::new(0, 4);
// pub const EMPTY: TileIndex = TileIndex::new(0, 5);
// pub const TODO: TileIndex = TileIndex::new(0, 6);
// pub const TODO: TileIndex = TileIndex::new(0, 7);
// pub const EMPTY: TileIndex = TileIndex::new(0, 8);
// pub const TODO: TileIndex = TileIndex::new(0, 9);
// pub const TODO: TileIndex = TileIndex::new(0, 10);
// pub const TODO: TileIndex = TileIndex::new(0, 11);
// pub const TODO: TileIndex = TileIndex::new(0, 12);
// pub const TODO: TileIndex = TileIndex::new(0, 13);
// pub const TODO: TileIndex = TileIndex::new(0, 14);
// pub const TODO: TileIndex = TileIndex::new(0, 15);
// pub const TODO: TileIndex = TileIndex::new(0, 16);
// pub const TODO: TileIndex = TileIndex::new(1, 0);
// pub const TODO: TileIndex = TileIndex::new(1, 1);
// pub const TODO: TileIndex = TileIndex::new(1, 2);
// pub const TODO: TileIndex = TileIndex::new(1, 3);
// pub const TODO: TileIndex = TileIndex::new(1, 4);
// pub const TODO: TileIndex = TileIndex::new(1, 5);
// pub const TODO: TileIndex = TileIndex::new(1, 6);
// pub const TODO: TileIndex = TileIndex::new(1, 7);
// pub const TODO: TileIndex = TileIndex::new(1, 8);
// pub const TODO: TileIndex = TileIndex::new(1, 9);
// pub const TODO: TileIndex = TileIndex::new(1, 10);
// pub const TODO: TileIndex = TileIndex::new(1, 11);
// pub const TODO: TileIndex = TileIndex::new(1, 12);
// pub const TODO: TileIndex = TileIndex::new(1, 13);
// pub const TODO: TileIndex = TileIndex::new(1, 14);
// pub const TODO: TileIndex = TileIndex::new(1, 15);
// pub const TODO: TileIndex = TileIndex::new(1, 16);
// pub const TODO: TileIndex = TileIndex::new(2, 0);
// pub const TODO: TileIndex = TileIndex::new(2, 1);
// pub const TODO: TileIndex = TileIndex::new(2, 2);
// pub const TODO: TileIndex = TileIndex::new(2, 3);
// pub const TODO: TileIndex = TileIndex::new(2, 4);
// pub const TODO: TileIndex = TileIndex::new(2, 5);
// pub const TODO: TileIndex = TileIndex::new(2, 6);
// pub const TODO: TileIndex = TileIndex::new(2, 7);
// pub const TODO: TileIndex = TileIndex::new(2, 8);
// pub const TODO: TileIndex = TileIndex::new(2, 9);
// pub const TODO: TileIndex = TileIndex::new(2, 10);
// pub const TODO: TileIndex = TileIndex::new(2, 11);
// pub const TODO: TileIndex = TileIndex::new(2, 12);
// pub const TODO: TileIndex = TileIndex::new(2, 13);
// pub const TODO: TileIndex = TileIndex::new(2, 14);
// pub const TODO: TileIndex = TileIndex::new(2, 15);
// pub const TODO: TileIndex = TileIndex::new(2, 16);
// pub const TODO: TileIndex = TileIndex::new(3, 0);
// pub const TODO: TileIndex = TileIndex::new(3, 1);
// pub const TODO: TileIndex = TileIndex::new(3, 2);
// pub const TODO: TileIndex = TileIndex::new(3, 3);
// pub const TODO: TileIndex = TileIndex::new(3, 4);
// pub const TODO: TileIndex = TileIndex::new(3, 5);
// pub const TODO: TileIndex = TileIndex::new(3, 6);
// pub const TODO: TileIndex = TileIndex::new(3, 7);
// pub const TODO: TileIndex = TileIndex::new(3, 8);
// pub const TODO: TileIndex = TileIndex::new(3, 9);
// pub const TODO: TileIndex = TileIndex::new(3, 10);
// pub const TODO: TileIndex = TileIndex::new(3, 11);
// pub const TODO: TileIndex = TileIndex::new(3, 12);
// pub const TODO: TileIndex = TileIndex::new(3, 13);
// pub const TODO: TileIndex = TileIndex::new(3, 14);
// pub const TODO: TileIndex = TileIndex::new(3, 15);
// pub const TODO: TileIndex = TileIndex::new(3, 16);
pub const TOPBOTTOM_LEFT_LEDGE: TileIndex = TileIndex::new(4, 0);
pub const TOPBOTTOM_MIDDLE_LEDGE: TileIndex = TileIndex::new(4, 1);
pub const TOPBOTTOM_RIGHT_LEDGE: TileIndex = TileIndex::new(4, 2);
// pub const EMPTY: TileIndex = TileIndex::new(4, 3);
pub const TOPBOTTOM_LEFTRIGHT_LEDGE: TileIndex = TileIndex::new(4, 4);
// pub const EMPTY: TileIndex = TileIndex::new(4, 5);
// pub const TODO: TileIndex = TileIndex::new(4, 6);
// pub const TODO: TileIndex = TileIndex::new(4, 7);
// pub const TODO: TileIndex = TileIndex::new(4, 8);
// pub const TODO: TileIndex = TileIndex::new(4, 9);
// pub const TODO: TileIndex = TileIndex::new(4, 10);
// pub const TODO: TileIndex = TileIndex::new(4, 11);
// pub const TODO: TileIndex = TileIndex::new(4, 12);
// pub const TODO: TileIndex = TileIndex::new(4, 13);
// pub const TODO: TileIndex = TileIndex::new(4, 14);
// pub const TODO: TileIndex = TileIndex::new(4, 15);
// pub const TODO: TileIndex = TileIndex::new(4, 16);
