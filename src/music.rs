use bevy::app::App;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioSource, AudioTween};

use crate::entities::EntityID;
use crate::entities::player::{Player, PlayerSize};
use crate::params;
use crate::screens::Sounds;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayBGMEvent>()
            .add_event::<PlaySFXEvent>()
            .add_systems(Update, (
                update,
                change_size
            ).run_if(resource_exists::<Sounds>()))
        ;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BGM {
    Caves,
}

impl BGM {
    fn source(&self, sounds: &Sounds, size: &PlayerSize) -> Handle<AudioSource> {
        match self {
            BGM::Caves => match size {
                PlayerSize::S => sounds.caves_s.clone(),
                PlayerSize::M => sounds.caves_m.clone(),
                PlayerSize::L => sounds.caves_m.clone(),
            }
        }
    }
}

#[derive(Copy, Clone)]
pub enum SFX {
    Select,
}

impl SFX {
    // fn source(&self, sounds: &Sounds) -> Handle<AudioSource> {
    //     match self {
    //         SFX::Select => sounds.select.clone(),
    //     }
    // }

    fn volume(&self) -> f32 {
        match self {
            _ => 0.35,
        }
    }
}

#[derive(Event)]
pub struct PlayBGMEvent(pub BGM);

#[derive(Event)]
pub struct PlaySFXEvent(pub SFX);


#[derive(Resource)]
struct BGMInstance(BGM, PlayerSize, Handle<AudioInstance>);

fn update(
    mut commands: Commands,
    mut bgm_event: EventReader<PlayBGMEvent>,
    mut sfx_event: EventReader<PlaySFXEvent>,
    player: Query<&EntityID, With<Player>>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    mut bgm_instance: Option<ResMut<BGMInstance>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let size = match player.get_single() {
        Ok(EntityID::Player(size)) => size,
        _ => &PlayerSize::M,
    };

    // SFX
    // for PlaySFXEvent(sfx) in sfx_event.iter() {
    //     // audio.play(sfx.source(&sounds));
    // }

    // BGM
    for PlayBGMEvent(bgm) in bgm_event.iter() {
        if let Some(ref mut instance) = bgm_instance {
            if let Some(mut handle) = audio_instances.get_mut(&instance.2) {
                handle.stop(AudioTween::default());
                instance.0 = bgm.clone();
                instance.1 = size.clone();
                instance.2 = audio
                    .play(bgm.source(&sounds, size))
                    .handle()
                ;
            } else {
                error!("No handle for bgm channel");
            }
        } else {
            let handle = audio
                .play(bgm.source(&sounds, size))
                .handle()
            ;
            commands
                .insert_resource(BGMInstance(bgm.clone(), size.clone(), handle))
            ;
        }
    }
}

fn change_size(
    player: Query<&EntityID, (With<Player>, Changed<EntityID>)>,
    audio: Res<Audio>,
    sounds: Res<Sounds>,
    mut bgm_instance: Option<ResMut<BGMInstance>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let Ok(EntityID::Player(size)) = player.get_single() else { return };

    if let Some(ref mut instance) = bgm_instance {
        if instance.1 == *size { return; }
        if let Some(mut handle) = audio_instances.get_mut(&instance.2) {
            let Some(position) = handle.state().position() else { return };
            handle.stop(AudioTween::default());
            let h = audio
                .play(instance.0.source(&sounds, size))
                .with_volume(params::BGM_VOLUME)
                .start_from(position)
                .handle()
            ;
            instance.1 = *size;
            instance.2 = h;
        }
    }
}