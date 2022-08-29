use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use bevy_kira_audio::AudioSource;

use crate::state::AppState;

pub struct Loading;

#[derive(Default)]
pub struct Sprites {
    pub font: Handle<Font>,
    pub cursor: Handle<Image>,
    //asset_server.load("spaceshooter/PNG/Lasers/laserRed05.png"),
    pub laser: Handle<Image>,
    //texture: asset_server.load("spaceshooter/PNG/Meteors/meteorBrown_big1.png"),
    pub meteor: Handle<Image>,
    //asset_server.load("spaceshooter/Backgrounds/black.png");
    pub bg: Handle<Image>,
    // texture: asset_server.load("spaceshooter/PNG/Lasers/laserBlue08.png"),
    pub explosion: Handle<Image>,
    pub main_menu_ost: Handle<AudioSource>,
    pub editors_ost: Handle<AudioSource>,
    pub mining_ost: Handle<AudioSource>,
}

impl Plugin for Loading {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetsLoading>()
            .init_resource::<Sprites>()
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_loading));
    }
}

#[derive(Default, Deref, DerefMut)]
struct AssetsLoading(Vec<HandleUntyped>);

fn load(
    server: Res<AssetServer>,
    mut assets_loading: ResMut<AssetsLoading>,
    mut sprites: ResMut<Sprites>,
) {
    sprites.font = {
        let handle = server.load("fonts/Kenney Future.ttf");
        assets_loading.push(handle.clone_untyped());
        handle
    };

    sprites.cursor = {
        let handle = server.load("spaceshooter/PNG/UI/cursor.png");
        assets_loading.push(handle.clone_untyped());
        handle
    };

    sprites.laser = {
        let handle = server.load("spaceshooter/PNG/Lasers/laserRed05.png");
        assets_loading.push(handle.clone_untyped());
        handle
    };

    sprites.meteor = {
        let handle = server.load("spaceshooter/PNG/Meteors/meteorBrown_big1.png");
        assets_loading.push(handle.clone_untyped());
        handle
    };
    //asset_server.load("spaceshooter/Backgrounds/black.png"
    sprites.bg = {
        let handle = server.load("spaceshooter/Backgrounds/black.png");
        assets_loading.push(handle.clone_untyped());
        handle
    };

    sprites.explosion = {
        let handle = server.load("spaceshooter/PNG/Lasers/laserBlue08.png");
        assets_loading.push(handle.clone_untyped());
        handle
    };

    sprites.main_menu_ost = {
        let handle = server.load("title_screen.ogg");
        assets_loading.push(handle.clone_untyped());
        handle
    };

    sprites.editors_ost = {
        let handle = server.load("battle.ogg");
        assets_loading.push(handle.clone_untyped());
        handle
    };

    sprites.mining_ost = {
        let handle = server.load("shop_screen.ogg");
        assets_loading.push(handle.clone_untyped());
        handle
    };
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
            state.set(AppState::MainMenu).unwrap();
        }
        LoadState::NotLoaded | LoadState::Loading | LoadState::Unloaded => {}
        LoadState::Failed => {
            unimplemented!()
        }
    }
}
