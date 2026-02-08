use bevy::prelude::*;

/// Game state enum
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
    LevelClear,
    Settings,
}
