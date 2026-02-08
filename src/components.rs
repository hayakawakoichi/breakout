use bevy::prelude::*;

/// Paddle marker component
#[derive(Component)]
pub struct Paddle;

/// Ball marker component
#[derive(Component)]
pub struct Ball;

/// Velocity component for moving entities
#[derive(Component)]
pub struct Velocity(pub Vec2);

/// Block type variants
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockType {
    Normal,
    Durable { hits_remaining: u32 },
    Steel,
    Explosive,
}

/// Block component with type information
#[derive(Component)]
pub struct Block {
    pub block_type: BlockType,
}

/// Combo popup UI marker
#[derive(Component)]
pub struct ComboPopup {
    pub timer: Timer,
}

/// Wall type for collision handling
#[derive(Component)]
pub enum Wall {
    Top,
    Left,
    Right,
    Bottom,
}

/// Collider component with size for AABB collision
#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

/// Score text UI marker
#[derive(Component)]
pub struct ScoreText;

/// Level text UI marker
#[derive(Component)]
pub struct LevelText;

/// Menu UI marker
#[derive(Component)]
pub struct MenuUI;

/// Game over UI marker
#[derive(Component)]
pub struct GameOverUI;

/// Level clear UI marker
#[derive(Component)]
pub struct LevelClearUI;

/// BGM music marker component
#[derive(Component)]
pub struct BgmMusic;

/// Particle component for block destruction effects and ball trail
#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub velocity: Vec2,
    pub initial_alpha: f32,
}

/// Power-up item types
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PowerUpType {
    WidePaddle,
    MultiBall,
    SlowBall,
    FireBall,
}

/// Marker component for falling power-up items
#[derive(Component)]
pub struct PowerUp {
    pub power_type: PowerUpType,
}

/// Single active power-up effect entry
pub struct ActiveEffect {
    pub effect_type: PowerUpType,
    pub timer: Timer,
}

/// High score text UI marker
#[derive(Component)]
pub struct HighScoreText;

/// NEW RECORD flash component
#[derive(Component)]
pub struct NewRecordFlash {
    pub timer: Timer,
}

/// Animated rank marker (bouncing ◀)
#[derive(Component)]
pub struct RankMarker(pub f32);

/// Active power-up effects attached to paddle (supports multiple simultaneous effects)
#[derive(Component, Default)]
pub struct PowerUpEffects {
    pub effects: Vec<ActiveEffect>,
}

/// Settings screen UI marker
#[derive(Component)]
pub struct SettingsUI;

/// Settings screen: currently selected row (0=BGM, 1=SFX)
#[derive(Resource)]
pub struct SettingsSelection {
    pub index: usize,
}

impl Default for SettingsSelection {
    fn default() -> Self {
        Self { index: 0 }
    }
}

/// Settings screen: BGM volume text marker
#[derive(Component)]
pub struct SettingsBgmText;

/// Settings screen: SFX volume text marker
#[derive(Component)]
pub struct SettingsSfxText;

/// Settings screen: selection cursor marker
#[derive(Component)]
pub struct SettingsCursor;

/// Menu screen: tappable settings button marker
#[derive(Component)]
pub struct SettingsButton;

/// Pause overlay UI marker
#[derive(Component)]
pub struct PauseUI;

/// HUD pause button marker
#[derive(Component)]
pub struct PauseButton;

/// Countdown container component (attached to parent node)
#[derive(Component)]
pub struct CountdownDisplay {
    pub timer: Timer,
    pub count: u32,
}

/// Countdown text marker (attached to child text entity for animation)
#[derive(Component)]
pub struct CountdownText;
