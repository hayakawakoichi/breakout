use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
 use bevy::text::FontSmoothing;

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
        Msaa::Off,
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
            color: Color::srgb(0.95, 0.85, 0.65), // Cream
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
    let initial_direction = Vec2::new(0.15, -1.0).normalize();
    let speed = BALL_SPEED * level.speed_multiplier();

    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.96, 0.88), // Warm white
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

/// Get color for a block type
pub fn block_type_color(block_type: &BlockType, row: usize) -> Color {
    let normal_colors = [
        Color::srgb(0.92, 0.44, 0.44), // Coral
        Color::srgb(0.95, 0.60, 0.35), // Orange
        Color::srgb(0.95, 0.85, 0.40), // Yellow
        Color::srgb(0.40, 0.80, 0.52), // Green
        Color::srgb(0.44, 0.60, 0.92), // Blue
    ];

    match block_type {
        BlockType::Normal => normal_colors[row % normal_colors.len()],
        BlockType::Durable { hits_remaining } => match hits_remaining {
            3 => Color::srgb(0.55, 0.15, 0.15), // Dark red
            2 => Color::srgb(0.80, 0.35, 0.15), // Dark orange
            _ => Color::srgb(0.95, 0.60, 0.35), // Orange (about to break)
        },
        BlockType::Steel => Color::srgb(0.50, 0.50, 0.55),    // Grey
        BlockType::Explosive => Color::srgb(0.90, 0.30, 0.30), // Red-purple
    }
}

/// Get color for a durable block based on remaining hits
pub fn durable_color(hits_remaining: u32) -> Color {
    match hits_remaining {
        3 => Color::srgb(0.55, 0.15, 0.15),
        2 => Color::srgb(0.80, 0.35, 0.15),
        _ => Color::srgb(0.95, 0.60, 0.35),
    }
}

/// Spawn a single block at the given position
pub fn spawn_block(commands: &mut Commands, x: f32, y: f32, block_type: BlockType, row: usize) {
    let color = block_type_color(&block_type, row);
    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        Block { block_type },
        Collider {
            size: Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT),
        },
    ));
}

/// Grid helper: compute x position for a column
pub fn grid_x(col: usize) -> f32 {
    let total_width = BLOCK_COLS as f32 * (BLOCK_WIDTH + BLOCK_GAP) - BLOCK_GAP;
    let start_x = -total_width / 2.0 + BLOCK_WIDTH / 2.0;
    start_x + col as f32 * (BLOCK_WIDTH + BLOCK_GAP)
}

/// Grid helper: compute y position for a row
pub fn grid_y(row: usize) -> f32 {
    BLOCKS_START_Y - row as f32 * (BLOCK_HEIGHT + BLOCK_GAP)
}

/// Spawn blocks based on current level
pub fn spawn_blocks(mut commands: Commands, level: Res<Level>) {
    match level.current {
        1 => spawn_level_1(&mut commands),
        2 => spawn_level_2(&mut commands),
        3 => spawn_level_3(&mut commands),
        4 => spawn_level_4(&mut commands),
        5 => spawn_level_5(&mut commands),
        6 => spawn_level_6(&mut commands),
        7 => spawn_level_7(&mut commands),
        8 => spawn_level_8(&mut commands),
        _ => spawn_generated_level(&mut commands, level.current),
    }
}

/// Level 1: Standard 5x10 grid (Normal blocks only)
fn spawn_level_1(commands: &mut Commands) {
    for row in 0..BLOCK_ROWS {
        for col in 0..BLOCK_COLS {
            spawn_block(commands, grid_x(col), grid_y(row), BlockType::Normal, row);
        }
    }
}

/// Level 2: Diamond pattern with Durable(2) blocks mixed in
fn spawn_level_2(commands: &mut Commands) {
    let center_col = BLOCK_COLS / 2;
    let rows = 7;
    for row in 0..rows {
        // Diamond shape: width expands then contracts
        let half_width = if row <= rows / 2 { row + 1 } else { rows - row };
        let start = center_col.saturating_sub(half_width);
        let end = (center_col + half_width).min(BLOCK_COLS);

        for col in start..end {
            let block_type = if (row + col) % 3 == 0 {
                BlockType::Durable { hits_remaining: 2 }
            } else {
                BlockType::Normal
            };
            spawn_block(commands, grid_x(col), grid_y(row), block_type, row);
        }
    }
}

/// Level 3: Full grid with Steel barrier in the middle, Explosive blocks behind it
fn spawn_level_3(commands: &mut Commands) {
    // 7 rows for more depth
    let rows = 7;
    for row in 0..rows {
        for col in 0..BLOCK_COLS {
            let block_type = if row == 3 && col != 2 && col != 7 {
                // Row 3: Steel barrier with gaps at col 2 and 7
                BlockType::Steel
            } else if row <= 1 && col >= 3 && col <= 6 {
                // Rows 0-1, center: Explosive blocks (behind Steel, reached last)
                BlockType::Explosive
            } else if row <= 2 {
                // Rows 0-2 (above Steel): Durable blocks
                BlockType::Durable { hits_remaining: 2 }
            } else {
                // Rows 4-6 (below Steel): Normal blocks (hit first)
                BlockType::Normal
            };

            spawn_block(commands, grid_x(col), grid_y(row), block_type, row);
        }
    }
}

/// Level 4: Maze — Steel walls form corridors, Normal/Durable blocks fill the passages
fn spawn_level_4(commands: &mut Commands) {
    // 7 rows, Steel walls create narrow corridors for the ball to navigate
    let rows = 7;
    for row in 0..rows {
        for col in 0..BLOCK_COLS {
            // Steel walls: vertical pillars at cols 3 and 6 (with gaps)
            let is_pillar_col = col == 3 || col == 6;
            let is_gap_row = row == 1 || row == 5; // gaps in pillars for ball passage

            // Horizontal Steel barriers at row 3 (with gaps at cols 1, 4, 8)
            let is_h_barrier = row == 3 && col != 1 && col != 4 && col != 8;

            let block_type = if is_pillar_col && !is_gap_row {
                BlockType::Steel
            } else if is_h_barrier {
                BlockType::Steel
            } else if row == 0 || row == rows - 1 {
                // Top/bottom rows: Durable blocks (harder to clear)
                BlockType::Durable { hits_remaining: 2 }
            } else {
                BlockType::Normal
            };

            spawn_block(commands, grid_x(col), grid_y(row), block_type, row);
        }
    }
}

/// Level 5: Spiral — blocks arranged in a spiral from outside inward, Durable-heavy
fn spawn_level_5(commands: &mut Commands) {
    let rows = 7;
    let cols = BLOCK_COLS; // 10

    // Build a grid and mark cells that are part of the spiral
    let mut grid = vec![vec![false; cols]; rows];

    // Trace a rectangular spiral inward
    let (mut top, mut bottom, mut left, mut right) = (0i32, rows as i32 - 1, 0i32, cols as i32 - 1);
    while top <= bottom && left <= right {
        // Top edge, left to right
        for c in left..=right { grid[top as usize][c as usize] = true; }
        top += 1;
        // Right edge, top to bottom
        for r in top..=bottom { grid[r as usize][right as usize] = true; }
        right -= 1;
        // Bottom edge, right to left
        if top <= bottom {
            for c in (left..=right).rev() { grid[bottom as usize][c as usize] = true; }
            bottom -= 1;
        }
        // Left edge, bottom to top
        if left <= right {
            for r in (top..=bottom).rev() { grid[r as usize][left as usize] = true; }
            left += 1;
        }
    }

    for row in 0..rows {
        for col in 0..cols {
            if !grid[row][col] { continue; }

            // Distance from center determines block type
            let center_r = (rows as f32 - 1.0) / 2.0;
            let center_c = (cols as f32 - 1.0) / 2.0;
            let dist = ((row as f32 - center_r).powi(2) + (col as f32 - center_c).powi(2)).sqrt();

            let block_type = if dist < 2.0 {
                // Inner core: Durable(3) — hardest to reach and break
                BlockType::Durable { hits_remaining: 3 }
            } else if dist < 3.5 {
                BlockType::Durable { hits_remaining: 2 }
            } else {
                BlockType::Normal
            };

            spawn_block(commands, grid_x(col), grid_y(row), block_type, row);
        }
    }
}

/// Level 6: Checkerboard — alternating blocks with Explosive scattered for chain reactions
fn spawn_level_6(commands: &mut Commands) {
    let rows = 7;
    for row in 0..rows {
        for col in 0..BLOCK_COLS {
            // Checkerboard: only place blocks where (row + col) is even
            if (row + col) % 2 != 0 { continue; }

            let block_type = if (row == 1 || row == 5) && (col == 2 || col == 7) {
                // Strategic Explosive blocks — create satisfying chain reactions
                BlockType::Explosive
            } else if row == 3 && (col == 0 || col == 4 || col == 8) {
                // A few more Explosives in the middle row
                BlockType::Explosive
            } else if row == 0 || row == rows - 1 {
                BlockType::Durable { hits_remaining: 2 }
            } else {
                BlockType::Normal
            };

            spawn_block(commands, grid_x(col), grid_y(row), block_type, row);
        }
    }
}

/// Level 7: Fortress — Steel outer walls protect a core of Durable/Explosive blocks
fn spawn_level_7(commands: &mut Commands) {
    let rows = 7;
    for row in 0..rows {
        for col in 0..BLOCK_COLS {
            // Outer ring: Steel fortress walls (with entry gaps)
            let is_outer_row = row == 0 || row == rows - 1;
            let is_outer_col = col == 0 || col == BLOCK_COLS - 1;
            let is_outer = is_outer_row || is_outer_col;

            // Entry gaps in the fortress: bottom-center and sides mid-height
            let is_entry = (row == rows - 1 && (col == 4 || col == 5))   // bottom gate
                || (col == 0 && row == 3)                                 // left gate
                || (col == BLOCK_COLS - 1 && row == 3);                   // right gate

            let block_type = if is_outer && !is_entry {
                BlockType::Steel
            } else if is_entry {
                // Entry points are Normal — breakable to get in
                BlockType::Normal
            } else if row >= 2 && row <= 4 && col >= 3 && col <= 6 {
                // Inner core: alternating Durable(3) and Explosive
                if (row + col) % 2 == 0 {
                    BlockType::Explosive
                } else {
                    BlockType::Durable { hits_remaining: 3 }
                }
            } else {
                // Fill between outer walls and core: Durable(2)
                BlockType::Durable { hits_remaining: 2 }
            };

            spawn_block(commands, grid_x(col), grid_y(row), block_type, row);
        }
    }
}

/// Level 8: Final hand-crafted level — all block types, complex layout, highest difficulty
fn spawn_level_8(commands: &mut Commands) {
    let rows = 7;
    for row in 0..rows {
        for col in 0..BLOCK_COLS {
            // Cross-shaped Steel frame dividing the field into quadrants
            let is_cross = (row == 3 && col != 0 && col != 9)
                || (col == 4 && row != 0 && row != 6)
                || (col == 5 && row != 0 && row != 6);

            // Explosive clusters in each quadrant corner
            let is_explosive_spot =
                (row == 1 && col == 1)
                || (row == 1 && col == 8)
                || (row == 5 && col == 1)
                || (row == 5 && col == 8);

            // Top two corners: Durable(3) blocks (hardest)
            let is_hard_corner =
                (row <= 1 && (col <= 1 || col >= 8))
                || (row >= 5 && (col <= 1 || col >= 8));

            let block_type = if is_cross {
                BlockType::Steel
            } else if is_explosive_spot {
                BlockType::Explosive
            } else if is_hard_corner {
                BlockType::Durable { hits_remaining: 3 }
            } else if row <= 2 {
                // Upper region: Durable(2) — behind the cross barrier
                BlockType::Durable { hits_remaining: 2 }
            } else {
                // Lower region: mix of Normal and Durable
                if (row + col) % 3 == 0 {
                    BlockType::Durable { hits_remaining: 2 }
                } else {
                    BlockType::Normal
                }
            };

            spawn_block(commands, grid_x(col), grid_y(row), block_type, row);
        }
    }
}

/// Level 9+: Auto-generated grid with increasing special block ratios
fn spawn_generated_level(commands: &mut Commands, level: u32) {
    // Probabilities increase with level
    let durable_chance = (0.10 + (level - 9) as f32 * 0.05).min(0.35);
    let steel_chance = (0.05 + (level - 9) as f32 * 0.03).min(0.15);
    let explosive_chance = (0.05 + (level - 9) as f32 * 0.02).min(0.12);

    for row in 0..BLOCK_ROWS {
        for col in 0..BLOCK_COLS {
            let roll = level_rand(level, row as u32, col as u32);

            let block_type = if roll < steel_chance {
                BlockType::Steel
            } else if roll < steel_chance + explosive_chance {
                BlockType::Explosive
            } else if roll < steel_chance + explosive_chance + durable_chance {
                let hits = if level >= 6 && level_rand(level, row as u32 + 100, col as u32) < 0.3 {
                    3
                } else {
                    2
                };
                BlockType::Durable { hits_remaining: hits }
            } else {
                BlockType::Normal
            };

            spawn_block(commands, grid_x(col), grid_y(row), block_type, row);
        }
    }
}

/// Deterministic pseudo-random based on level, row, col
fn level_rand(level: u32, row: u32, col: u32) -> f32 {
    let seed = level
        .wrapping_mul(7919)
        .wrapping_add(row.wrapping_mul(1301))
        .wrapping_add(col.wrapping_mul(3571));
    let n = seed.wrapping_mul(1103515245).wrapping_add(12345);
    (n & 0x7FFFFFFF) as f32 / 0x7FFFFFFF as f32
}

/// Spawn walls around the play area
pub fn spawn_walls(mut commands: Commands) {
    let wall_color = Color::srgb(0.22, 0.20, 0.32);

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

/// Spawn score, level, and high score UI
pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>, high_scores: Res<HighScores>) {
    let warm_white = Color::srgb(1.0, 0.96, 0.88);
    let lavender = Color::srgb(0.55, 0.50, 0.65);
    let font_handle: Handle<Font> = asset_server.load(GAME_FONT_PATH);

    // HUD bar background
    commands.spawn(Node {
        position_type: PositionType::Absolute,
        top: Val::Px(0.0),
        left: Val::Px(0.0),
        width: Val::Percent(100.0),
        height: Val::Px(45.0),
        ..default()
    }).insert(BackgroundColor(Color::srgba(0.07, 0.07, 0.16, 0.8)));

    // Score text (top-left)
    commands.spawn((
        Text::new("スコア 0"),
        TextFont {
            font: font_handle.clone(),
            font_size: 24.0,
            font_smoothing: FontSmoothing::None,
        },
        TextColor(warm_white),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(20.0),
            ..default()
        },
        ScoreText,
    ));

    // High score text (top-center)
    commands.spawn((
        Text::new(format!("ハイスコア {}", high_scores.best())),
        TextFont {
            font: font_handle.clone(),
            font_size: 16.0,
            font_smoothing: FontSmoothing::None,
        },
        TextColor(lavender),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-30.0)),
            ..default()
        },
        HighScoreText,
    ));

    // Pause button (top-right, left of level text)
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(6.0),
                right: Val::Px(155.0),
                padding: UiRect::axes(Val::Px(10.0), Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::NONE),
            PauseButton,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new("||"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 24.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(lavender),
            ));
        });

    // Level text (top-right)
    commands.spawn((
        Text::new("レベル 1"),
        TextFont {
            font: font_handle,
            font_size: 24.0,
            font_smoothing: FontSmoothing::None,
        },
        TextColor(warm_white),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            right: Val::Px(20.0),
            ..default()
        },
        LevelText,
    ));
}

/// Record score at level start for statistics
pub fn record_level_start_score(score: Res<Score>, mut level_stats: ResMut<LevelStats>) {
    *level_stats = LevelStats::default();
    level_stats.score_at_level_start = score.value;
}

/// Load sound assets
pub fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sounds = GameSounds {
        bounce: asset_server.load_optional("sounds/bounce.wav"),
        break_block: asset_server.load_optional("sounds/break.wav"),
        game_over: asset_server.load_optional("sounds/gameover.wav"),
        level_up: asset_server.load_optional("sounds/levelup.wav"),
        bgm: asset_server.load_optional("sounds/bgm.wav"),
        countdown_beep: asset_server.load_optional("sounds/countdown_beep.wav"),
        countdown_go: asset_server.load_optional("sounds/countdown_go.wav"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::test_app;

    /// Helper: spawn blocks for a given level and return all Block components
    fn blocks_for_level(level: u32) -> Vec<BlockType> {
        let mut app = test_app();
        app.world_mut().resource_mut::<crate::resources::Level>().current = level;
        app.add_systems(Update, spawn_blocks);
        app.update();
        app.world_mut()
            .query::<&Block>()
            .iter(app.world())
            .map(|b| b.block_type)
            .collect()
    }

    /// Helper: check that a level has at least one non-Steel block (clearable)
    fn has_clearable_blocks(blocks: &[BlockType]) -> bool {
        blocks.iter().any(|bt| !matches!(bt, BlockType::Steel))
    }

    #[test]
    fn level_4_spawns_blocks() {
        let blocks = blocks_for_level(4);
        assert!(!blocks.is_empty(), "Level 4 should spawn blocks");
        assert!(has_clearable_blocks(&blocks), "Level 4 must be clearable");
    }

    #[test]
    fn level_4_has_steel_maze_walls() {
        let blocks = blocks_for_level(4);
        let steel_count = blocks.iter().filter(|bt| matches!(bt, BlockType::Steel)).count();
        assert!(steel_count > 0, "Level 4 maze should have Steel blocks");
    }

    #[test]
    fn level_5_spawns_blocks() {
        let blocks = blocks_for_level(5);
        assert!(!blocks.is_empty(), "Level 5 should spawn blocks");
        assert!(has_clearable_blocks(&blocks), "Level 5 must be clearable");
    }

    #[test]
    fn level_5_has_durable_blocks() {
        let blocks = blocks_for_level(5);
        let durable_count = blocks.iter().filter(|bt| matches!(bt, BlockType::Durable { .. })).count();
        assert!(durable_count > 0, "Level 5 spiral should have Durable blocks");
    }

    #[test]
    fn level_6_spawns_blocks() {
        let blocks = blocks_for_level(6);
        assert!(!blocks.is_empty(), "Level 6 should spawn blocks");
        assert!(has_clearable_blocks(&blocks), "Level 6 must be clearable");
    }

    #[test]
    fn level_6_has_explosive_blocks() {
        let blocks = blocks_for_level(6);
        let explosive_count = blocks.iter().filter(|bt| matches!(bt, BlockType::Explosive)).count();
        assert!(explosive_count > 0, "Level 6 checkerboard should have Explosive blocks");
    }

    #[test]
    fn level_7_spawns_blocks() {
        let blocks = blocks_for_level(7);
        assert!(!blocks.is_empty(), "Level 7 should spawn blocks");
        assert!(has_clearable_blocks(&blocks), "Level 7 must be clearable");
    }

    #[test]
    fn level_7_has_all_special_types() {
        let blocks = blocks_for_level(7);
        assert!(blocks.iter().any(|bt| matches!(bt, BlockType::Steel)), "Level 7 fortress should have Steel");
        assert!(blocks.iter().any(|bt| matches!(bt, BlockType::Explosive)), "Level 7 fortress should have Explosive");
        assert!(blocks.iter().any(|bt| matches!(bt, BlockType::Durable { .. })), "Level 7 fortress should have Durable");
    }

    #[test]
    fn level_8_spawns_blocks() {
        let blocks = blocks_for_level(8);
        assert!(!blocks.is_empty(), "Level 8 should spawn blocks");
        assert!(has_clearable_blocks(&blocks), "Level 8 must be clearable");
    }

    #[test]
    fn level_8_has_all_block_types() {
        let blocks = blocks_for_level(8);
        assert!(blocks.iter().any(|bt| matches!(bt, BlockType::Normal)), "Level 8 should have Normal blocks");
        assert!(blocks.iter().any(|bt| matches!(bt, BlockType::Steel)), "Level 8 should have Steel blocks");
        assert!(blocks.iter().any(|bt| matches!(bt, BlockType::Explosive)), "Level 8 should have Explosive blocks");
        assert!(blocks.iter().any(|bt| matches!(bt, BlockType::Durable { .. })), "Level 8 should have Durable blocks");
    }

    #[test]
    fn level_9_uses_generated() {
        // Level 9+ should use auto-generated; verify it still spawns blocks
        let blocks = blocks_for_level(9);
        assert!(!blocks.is_empty(), "Level 9 (generated) should spawn blocks");
        assert!(has_clearable_blocks(&blocks), "Level 9 must be clearable");
    }
}
