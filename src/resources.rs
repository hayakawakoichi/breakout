use bevy::prelude::*;

/// Score tracking resource
#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

/// Level tracking resource
#[derive(Resource)]
pub struct Level {
    pub current: u32,
}

impl Default for Level {
    fn default() -> Self {
        Self { current: 1 }
    }
}

impl Level {
    /// Get the ball speed multiplier based on current level
    pub fn speed_multiplier(&self) -> f32 {
        1.0 + (self.current - 1) as f32 * crate::constants::SPEED_INCREASE_PER_LEVEL
    }
}

/// Font path constant
pub const GAME_FONT_PATH: &str = "fonts/DotGothic16-Regular.ttf";

/// Sound handles resource
#[derive(Resource, Default)]
pub struct GameSounds {
    pub bounce: Option<Handle<AudioSource>>,
    pub break_block: Option<Handle<AudioSource>>,
    pub game_over: Option<Handle<AudioSource>>,
    pub level_up: Option<Handle<AudioSource>>,
}
