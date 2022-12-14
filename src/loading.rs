use crate::{screen_cleanup, Screen};
use bevy::prelude::*;
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
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(spawn_text))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(check_loading))
            .add_system_set(SystemSet::on_exit(AppState::Loading).with_system(screen_cleanup));
    }
}

#[derive(Default, Deref, DerefMut)]
struct AssetsLoading(Vec<HandleUntyped>);

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font: asset_server.load("fonts/Kenney Future.ttf"),
                    font_size: 50.0,
                    color: Color::ALICE_BLUE,
                },
            )
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                ..default()
            }),
        )
        .insert(Screen(AppState::Loading));
}

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

    sprites.mining_ost = {
        let handle = server.load("battle.ogg");
        assets_loading.push(handle.clone_untyped());
        handle
    };

    sprites.editors_ost = {
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
