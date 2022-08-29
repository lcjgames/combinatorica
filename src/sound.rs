use crate::{AppState, Sprites};
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_kira_audio::*;
use enum_iterator::IntoEnumIterator;

pub struct Sound;

impl Plugin for Sound {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin).init_resource::<Music>();
        for state in AppState::into_enum_iter() {
            app.add_system_set(SystemSet::on_enter(state).with_system(play_music));
        }
    }
}

fn get_state_song(state: &AppState, assets: &Sprites) -> Handle<AudioSource> {
    match state {
        AppState::MainMenu | AppState::Loading => assets.main_menu_ost.clone(),
        AppState::Battle => assets.mining_ost.clone(),
        AppState::FleetEditor | AppState::PartSelection | AppState::ShipEditor | AppState::Shop => {
            assets.editors_ost.clone()
        }
    }
}

struct MusicId {
    handle: Handle<AudioSource>,
    id: Handle<AudioInstance>,
}

#[derive(Default)]
struct Music(Option<MusicId>);

fn play_music(
    state: Res<State<AppState>>,
    assets: Res<Sprites>,
    audio: Res<Audio>,
    mut music: ResMut<Music>,
) {
    let handle = get_state_song(state.current(), &assets);
    if let Some(music_id) = &music.0 {
        if music_id.handle == handle {
            return;
        }
        audio.stop();
    }
    let id = audio.play(handle.clone()).looped().handle();
    music.0 = Some(MusicId { handle, id });
}
