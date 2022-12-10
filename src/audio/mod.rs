use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct AudioPlugin;

#[derive(Resource)]
pub struct BackgroundMusic;

#[derive(Resource)]
pub struct AudioState {
    pub bgm_handle: Handle<AudioSource>,
    pub volume: f64,
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system_to_stage(StartupStage::PreStartup, load_audio)
        //     .add_audio_channel::<BackgroundMusic>();
        app.add_audio_channel::<BackgroundMusic>();
    }
}

// fn load_audio(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     background: Res<AudioChannel<BackgroundMusic>>,
// ) {
//     let bgm_handle: Handle<AudioSource> = asset_server.load("audio/harmony.ogg");
//     let volume = 0.5;

//     background.set_volume(volume);

//     commands.insert_resource(AudioState { bgm_handle, volume });
// }
