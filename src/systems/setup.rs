use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// Setup the 2D camera with scaling to fit mobile screens
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: WINDOW_WIDTH,
                min_height: WINDOW_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}

/// Scale UI based on window width to prevent text overflow on small screens
pub fn update_ui_scale(
    windows: Query<&Window>,
    mut ui_scale: ResMut<UiScale>,
) {
    if let Ok(window) = windows.get_single() {
        let scale = (window.width() / WINDOW_WIDTH).clamp(0.5, 1.0);
        if (ui_scale.0 - scale).abs() > 0.01 {
            ui_scale.0 = scale;
        }
    }
}

/// Spawn the paddle
pub fn spawn_paddle(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.85, 1.0),
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
        Paddle,
        Collider {
            size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
        },
    ));
}

/// Spawn the ball
pub fn spawn_ball(mut commands: Commands, level: Res<Level>) {
    let initial_direction = Vec2::new(0.5, 0.5).normalize();
    let speed = BALL_SPEED * level.speed_multiplier();

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Ball,
        Velocity(initial_direction * speed),
        Collider {
            size: Vec2::new(BALL_SIZE, BALL_SIZE),
        },
    ));
}

/// Spawn blocks in a grid pattern
pub fn spawn_blocks(mut commands: Commands) {
    let colors = [
        Color::srgb(1.0, 0.1, 0.3),  // Neon Red
        Color::srgb(1.0, 0.0, 0.6),  // Neon Magenta
        Color::srgb(0.6, 0.0, 1.0),  // Neon Purple
        Color::srgb(0.0, 0.8, 1.0),  // Neon Cyan
        Color::srgb(0.0, 1.0, 0.5),  // Neon Green
    ];

    let total_width = BLOCK_COLS as f32 * (BLOCK_WIDTH + BLOCK_GAP) - BLOCK_GAP;
    let start_x = -total_width / 2.0 + BLOCK_WIDTH / 2.0;

    for row in 0..BLOCK_ROWS {
        for col in 0..BLOCK_COLS {
            let x = start_x + col as f32 * (BLOCK_WIDTH + BLOCK_GAP);
            let y = BLOCKS_START_Y - row as f32 * (BLOCK_HEIGHT + BLOCK_GAP);

            commands.spawn((
                Sprite {
                    color: colors[row % colors.len()],
                    custom_size: Some(Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
                Block,
                Collider {
                    size: Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT),
                },
            ));
        }
    }
}

/// Spawn walls around the play area
pub fn spawn_walls(mut commands: Commands) {
    let wall_color = Color::srgb(0.15, 0.15, 0.25);

    // Top wall
    commands.spawn((
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALL_THICKNESS)),
            ..default()
        },
        Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - WALL_THICKNESS / 2.0, 0.0),
        Wall::Top,
        Collider {
            size: Vec2::new(WINDOW_WIDTH, WALL_THICKNESS),
        },
    ));

    // Left wall
    commands.spawn((
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(-WINDOW_WIDTH / 2.0 + WALL_THICKNESS / 2.0, 0.0, 0.0),
        Wall::Left,
        Collider {
            size: Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT),
        },
    ));

    // Right wall
    commands.spawn((
        Sprite {
            color: wall_color,
            custom_size: Some(Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(WINDOW_WIDTH / 2.0 - WALL_THICKNESS / 2.0, 0.0, 0.0),
        Wall::Right,
        Collider {
            size: Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT),
        },
    ));

    // Bottom wall (invisible, for game over detection)
    commands.spawn((
        Sprite {
            color: Color::NONE,
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALL_THICKNESS)),
            ..default()
        },
        Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - WALL_THICKNESS / 2.0, 0.0),
        Wall::Bottom,
        Collider {
            size: Vec2::new(WINDOW_WIDTH, WALL_THICKNESS),
        },
    ));
}

/// Spawn score and level UI
pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cyan = Color::srgb(0.0, 0.9, 1.0);
    let font_handle: Handle<Font> = asset_server.load(GAME_FONT_PATH);

    // HUD bar background
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        left: Val::Px(0.0),
        width: Val::Percent(100.0),
        height: Val::Px(45.0),
        ..default()
    }).insert(BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)));

    // Score text
    commands.spawn((
        Text::new("スコア 0"),
        TextFont {
            font: font_handle.clone(),
            font_size: 28.0,
            ..default()
        },
        TextColor(cyan),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(20.0),
            ..default()
        },
        ScoreText,
    ));

    // Level text
    commands.spawn((
        Text::new("レベル 1"),
        TextFont {
            font: font_handle,
            font_size: 28.0,
            ..default()
        },
        TextColor(cyan),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            right: Val::Px(20.0),
            ..default()
        },
        LevelText,
    ));
}

/// Load sound assets
pub fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sounds = GameSounds {
        bounce: asset_server.load_optional("sounds/bounce.wav"),
        break_block: asset_server.load_optional("sounds/break.wav"),
        game_over: asset_server.load_optional("sounds/gameover.wav"),
        level_up: asset_server.load_optional("sounds/levelup.wav"),
    };
    commands.insert_resource(sounds);
}

/// Helper trait for optional asset loading
trait AssetServerExt {
    fn load_optional<A: bevy::asset::Asset>(&self, path: &str) -> Option<Handle<A>>;
}

impl AssetServerExt for AssetServer {
    fn load_optional<A: bevy::asset::Asset>(&self, path: &str) -> Option<Handle<A>> {
        // Try to load, return None if file doesn't exist
        Some(self.load(path))
    }
}
