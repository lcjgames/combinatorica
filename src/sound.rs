use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_kira_audio::*;
use enum_iterator::IntoEnumIterator;

pub struct Sound;

impl Plugin for Sound {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin).init_resource::<Music>();
        for state in AppState::into_enum_iter() {
            app.add_system_set(SystemSet::on_enter(state).with_system(play_song(state.into())));
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Song {
    MainTheme,
    BattleTheme,
    EditTheme,
}

impl From<AppState> for Song {
    fn from(state: AppState) -> Self {
        match state {
            AppState::MainMenu => Song::MainTheme,
            AppState::Battle => Song::BattleTheme,
            AppState::FleetEditor
            | AppState::PartSelection
            | AppState::ShipEditor
            | AppState::Shop => Song::EditTheme,
        }
    }
}

impl ToString for Song {
    fn to_string(&self) -> String {
        match self {
            Song::MainTheme => "title_screen.ogg".to_string(),
            Song::BattleTheme => "battle.ogg".to_string(),
            Song::EditTheme => "shop_screen.ogg".to_string(),
        }
    }
}

struct MusicId {
    song: Song,
    id: Handle<AudioInstance>,
}

#[derive(Default)]
struct Music(Option<MusicId>);

fn play_song(song: Song) -> impl Fn(Res<AssetServer>, Res<Audio>, ResMut<Music>) {
    move |asset_server: Res<AssetServer>, audio: Res<Audio>, mut music: ResMut<Music>| {
        if let Some(music_id) = &music.0 {
            if music_id.song == song {
                return;
            }
            audio.stop();
        }
        let id = audio
            .play(asset_server.load(song.to_string().as_str()))
            .looped()
            .handle();
        music.0 = Some(MusicId { song, id });
    }
}
