use bevy::prelude::*;

use crate::resources::*;

/// Collision event types for sound playback
#[derive(Event)]
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
            commands.spawn(AudioPlayer::new(source));
        }
    }
}
