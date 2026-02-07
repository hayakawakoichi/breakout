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

/// Block marker component
#[derive(Component)]
pub struct Block;

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

/// Particle component for block destruction effects
#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub velocity: Vec2,
}

/// Power-up item types
#[derive(Clone, Copy, PartialEq)]
pub enum PowerUpType {
    WidePaddle,
    MultiBall,
    SlowBall,
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

/// Active power-up effects attached to paddle (supports multiple simultaneous effects)
#[derive(Component, Default)]
pub struct PowerUpEffects {
    pub effects: Vec<ActiveEffect>,
}
