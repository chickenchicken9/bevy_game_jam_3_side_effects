use crate::loading::AudioAssets;
use crate::pill::spawn_pills;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
            .add_system(
                control_flying_sound
                    .after(spawn_pills)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct FlyingAudio {
    audio: Handle<AudioInstance>,
    _timer: Timer,
}

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(FlyingAudio {
        audio: handle,
        _timer: Timer::new(Duration::from_secs(5), TimerMode::Once),
    });
}

fn control_flying_sound(
    // actions: Res<Actions>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.audio) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                // if actions.player_movement.is_some() {
                //     instance.resume(AudioTween::default());
                // }
            }
            PlaybackState::Playing { .. } => {
                // if actions.player_movement.is_none() {
                //     instance.pause(AudioTween::default());
                // }
            }
            _ => {}
        }
    }
}
