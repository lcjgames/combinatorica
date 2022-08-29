use bevy::prelude::*;

use crate::state::AppState;

pub struct Loading;

#[derive(Default)]
pub struct Sprites {
    pub font: Handle<Font>,
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
    let mut load_asset = |path| {
        let handle = server.load(path);
        assets_loading.push(handle.clone_untyped());
        handle
    };
    sprites.font = load_asset("fonts/Kenney Future.ttf");
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
