// Window
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

// Paddle
pub const PADDLE_WIDTH: f32 = 100.0;
pub const PADDLE_HEIGHT: f32 = 20.0;
pub const PADDLE_SPEED: f32 = 500.0;
pub const PADDLE_Y: f32 = -350.0;

// Ball
pub const BALL_SIZE: f32 = 15.0;
pub const BALL_SPEED: f32 = 400.0;

// Blocks
pub const BLOCK_WIDTH: f32 = 70.0;
pub const BLOCK_HEIGHT: f32 = 25.0;
pub const BLOCK_ROWS: usize = 5;
pub const BLOCK_COLS: usize = 10;
pub const BLOCK_GAP: f32 = 5.0;
pub const BLOCKS_START_Y: f32 = 280.0;

// Walls
pub const WALL_THICKNESS: f32 = 10.0;

// Scoring
pub const SCORE_PER_BLOCK: u32 = 10;

// Level system
pub const SPEED_INCREASE_PER_LEVEL: f32 = 0.1;

// Particles
pub const PARTICLE_COUNT: usize = 6;
pub const PARTICLE_SIZE: f32 = 4.0;
pub const PARTICLE_SPEED: f32 = 200.0;
pub const PARTICLE_LIFETIME: f32 = 0.4;

// Ball trail
pub const TRAIL_SPAWN_INTERVAL: f32 = 0.05; // ~3 frames at 60fps
pub const TRAIL_PARTICLE_SIZE: f32 = 8.0;
pub const TRAIL_LIFETIME: f32 = 0.15;
pub const TRAIL_INITIAL_ALPHA: f32 = 0.5;

// Screen shake
pub const SHAKE_TRAUMA: f32 = 0.5;
pub const SHAKE_DECAY: f32 = 1.5;
pub const SHAKE_MAX_OFFSET: f32 = 8.0;

// Special blocks
pub const DURABLE_SCORE_BONUS: u32 = 5;
pub const EXPLOSIVE_RADIUS: f32 = 100.0;

// Combo system
pub const COMBO_WINDOW: f32 = 1.5;
pub const COMBO_POPUP_DURATION: f32 = 0.8;

// Power-ups
pub const POWERUP_DROP_CHANCE: f32 = 0.15;
pub const POWERUP_FALL_SPEED: f32 = 150.0;
pub const POWERUP_SIZE: f32 = 20.0;
pub const WIDE_PADDLE_DURATION: f32 = 8.0;
pub const WIDE_PADDLE_MULTIPLIER: f32 = 1.5;
pub const SLOW_BALL_DURATION: f32 = 6.0;
pub const SLOW_BALL_MULTIPLIER: f32 = 0.6;
pub const FIREBALL_DURATION: f32 = 8.0;

// Countdown
pub const COUNTDOWN_STEP_DURATION: f32 = 1.0;
pub const COUNTDOWN_GO_DURATION: f32 = 0.5;
pub const COUNTDOWN_SCALE_ANIM_DURATION: f32 = 0.3;

// Editor
pub const EDITOR_ROWS: usize = 7;
pub const EDITOR_COLS: usize = 10;
pub const EDITOR_CELL_SIZE: f32 = 60.0;
pub const EDITOR_CELL_GAP: f32 = 4.0;
