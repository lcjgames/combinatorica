use bevy::prelude::*;

use crate::sprites::{GroundTiles, SpriteSheets};
use crate::state::AppState;

pub struct Loading;

impl Plugin for Loading {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::Loading)
            .init_resource::<AssetsLoading>()
            .init_resource::<SpriteSheets>()
            .init_resource::<GroundTiles>()
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_loading));
    }
}

#[derive(Default, Deref, DerefMut)]
struct AssetsLoading(Vec<HandleUntyped>);

fn load(
    server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
    mut sprite_sheets: ResMut<SpriteSheets>,
    mut ground_tiles: ResMut<GroundTiles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut load_asset = |path| {
        let handle = server.load(path);
        assets_loading.push(handle.clone_untyped());
        handle
    };
    let mut load_sprite_sheet = |path| {
        let handle = load_asset(path);
        let texture_atlas = TextureAtlas::from_grid_with_padding(
            handle.clone(),
            Vec2::new(64.0, 64.0),
            14,
            7,
            Vec2::new(32.0, 16.0),
            Vec2::new(16.0, 0.0),
        );
        texture_atlases.add(texture_atlas)
    };
    sprite_sheets.player_fishy = load_sprite_sheet("spaceshooter/sample.png");
    sprite_sheets.player_orcy = load_sprite_sheet("spaceshooter/sample.png");
    sprite_sheets.player_pescy = load_sprite_sheet("spaceshooter/sample.png");
    // spritesheets.player_sharky = load_sprite_sheet("spaceshooter/sample.png");
    let mut load_ground_tiles = |path| {
        let handle = load_asset(path);
        let texture_atlas = TextureAtlas::from_grid(handle.clone(), Vec2::new(32.0, 32.0), 17, 5);
        texture_atlases.add(texture_atlas)
    };
    ground_tiles.metal = load_ground_tiles("spaceshooter/sample.png");
    ground_tiles.rock = load_ground_tiles("spaceshooter/sample.png");
    ground_tiles.wood = load_ground_tiles("spaceshooter/sample.png");
}

fn check_loading(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(loading.iter().map(|h| h.id)) {
        LoadState::Loaded => {
            commands.remove_resource::<AssetsLoading>();
            state.set(AppState::Game).unwrap();
        }
        LoadState::NotLoaded | LoadState::Loading | LoadState::Unloaded => {}
        LoadState::Failed => {
            unimplemented!()
        }
    }
}
