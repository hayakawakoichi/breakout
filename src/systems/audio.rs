use bevy::audio::Volume;
use bevy::prelude::*;

use crate::components::BgmMusic;
use crate::resources::*;

/// Collision event types for sound playback
#[derive(Event, Clone, Debug, PartialEq)]
pub enum CollisionEvent {
    Paddle,
    Wall,
    Block,
    GameOver,
    LevelClear,
}

/// Play sounds based on collision events
pub fn play_collision_sounds(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    sounds: Res<GameSounds>,
) {
    for event in events.read() {
        let sound = match event {
            CollisionEvent::Paddle | CollisionEvent::Wall => sounds.bounce.clone(),
            CollisionEvent::Block => sounds.break_block.clone(),
            CollisionEvent::GameOver => sounds.game_over.clone(),
            CollisionEvent::LevelClear => sounds.level_up.clone(),
        };

        if let Some(source) = sound {
            commands.spawn((
                AudioPlayer::new(source),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

/// Start BGM when entering Playing state
pub fn start_bgm(mut commands: Commands, sounds: Res<GameSounds>) {
    if let Some(source) = sounds.bgm.clone() {
        commands.spawn((
            AudioPlayer::new(source),
            PlaybackSettings::LOOP.with_volume(Volume::new(0.4)),
            BgmMusic,
        ));
    }
}

/// Stop BGM by despawning the BGM entity
pub fn stop_bgm(mut commands: Commands, bgm_query: Query<Entity, With<BgmMusic>>) {
    for entity in bgm_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Pause BGM
pub fn pause_bgm(bgm_query: Query<&AudioSink, With<BgmMusic>>) {
    for sink in bgm_query.iter() {
        sink.pause();
    }
}

/// Resume BGM
pub fn resume_bgm(bgm_query: Query<&AudioSink, With<BgmMusic>>) {
    for sink in bgm_query.iter() {
        sink.play();
    }
}
