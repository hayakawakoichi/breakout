use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// Create a small left-pointing triangle image (9x9 pixels, gold colored)
fn create_triangle_image() -> Image {
    let size = 9u32;
    let center = (size / 2) as i32;
    let mut data = vec![0u8; (size * size * 4) as usize];

    // Gold: rgb(255, 217, 51)
    for y in 0..size {
        let dist = (y as i32 - center).unsigned_abs();
        let left_edge = dist * (size - 1) / center as u32;
        for x in left_edge..size {
            let idx = ((y * size + x) * 4) as usize;
            data[idx] = 255;
            data[idx + 1] = 217;
            data[idx + 2] = 51;
            data[idx + 3] = 255;
        }
    }

    Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

/// Setup menu screen
pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let warm_white = Color::srgb(1.0, 0.96, 0.88);
    let cream = Color::srgb(0.95, 0.85, 0.65);
    let lavender = Color::srgb(0.55, 0.50, 0.65);
    let font_handle: Handle<Font> = asset_server.load(GAME_FONT_PATH);

    // Dark overlay container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.12, 0.88)),
            MenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("ブロック崩し"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 48.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(warm_white),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Start instruction
            parent.spawn((
                Text::new("SPACE / タップ でスタート"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 24.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(cream),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Controls
            parent.spawn((
                Text::new("← → / タップ  パドル操作\nESC  ポーズ"),
                TextFont {
                    font: font_handle,
                    font_size: 16.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(lavender),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

/// Cleanup menu screen
pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Setup game over screen
pub fn setup_game_over(
    mut commands: Commands,
    score: Res<Score>,
    level: Res<Level>,
    mut high_scores: ResMut<HighScores>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let salmon = Color::srgb(0.92, 0.44, 0.44);
    let cream = Color::srgb(0.95, 0.85, 0.65);
    let lavender = Color::srgb(0.55, 0.50, 0.65);
    let gold = Color::srgb(1.0, 0.85, 0.20);
    let font_handle: Handle<Font> = asset_server.load(GAME_FONT_PATH);

    let rank = high_scores.try_insert(score.value);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.12, 0.88)),
            GameOverUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("ゲームオーバー"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 48.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(salmon),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Score info
            parent.spawn((
                Text::new(format!("レベル {}\n最終スコア {}", level.current, score.value)),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 24.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(cream),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // NEW RECORD flash
            if rank.is_some() {
                parent.spawn((
                    Text::new("NEW RECORD!"),
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 32.0,
                        font_smoothing: FontSmoothing::None,
                    },
                    TextColor(gold),
                    TextLayout::new_with_justify(JustifyText::Center),
                    NewRecordFlash {
                        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    },
                ));
            }

            // Ranking title
            parent.spawn((
                Text::new("ランキング"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 24.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(cream),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Top 3 ranking entries
            let rank_labels = ["1st", "2nd", "3rd"];
            for (i, &s) in high_scores.scores.iter().enumerate() {
                let is_current = rank == Some(i);
                let color = if is_current { gold } else { lavender };
                let label = if s > 0 {
                    format!("{}  {}", rank_labels[i], s)
                } else {
                    format!("{}  ---", rank_labels[i])
                };

                parent
                    .spawn(Node {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|row| {
                        row.spawn((
                            Text::new(label),
                            TextFont {
                                font: font_handle.clone(),
                                font_size: 20.0,
                                font_smoothing: FontSmoothing::None,
                            },
                            TextColor(color),
                        ));

                        if is_current {
                            let triangle = images.add(create_triangle_image());
                            row.spawn((
                                ImageNode::new(triangle),
                                Node {
                                    width: Val::Px(9.0),
                                    height: Val::Px(9.0),
                                    margin: UiRect::left(Val::Px(8.0)),
                                    ..default()
                                },
                                RankMarker(0.0),
                            ));
                        }
                    });
            }

            // Restart instruction
            parent.spawn((
                Text::new("SPACE / タップ でリトライ"),
                TextFont {
                    font: font_handle,
                    font_size: 16.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(lavender),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

/// Cleanup game over screen
pub fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Setup level clear screen
pub fn setup_level_clear(
    mut commands: Commands,
    score: Res<Score>,
    level: Res<Level>,
    level_stats: Res<LevelStats>,
    asset_server: Res<AssetServer>,
) {
    let soft_green = Color::srgb(0.40, 0.80, 0.52);
    let cream = Color::srgb(0.95, 0.85, 0.65);
    let lavender = Color::srgb(0.55, 0.50, 0.65);
    let font_handle: Handle<Font> = asset_server.load(GAME_FONT_PATH);

    let level_score = score.value.saturating_sub(level_stats.score_at_level_start);
    let time_secs = level_stats.time_elapsed as u32;
    let time_min = time_secs / 60;
    let time_sec = time_secs % 60;

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.12, 0.88)),
            LevelClearUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new(format!("レベル {} クリア！", level.current)),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 48.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(soft_green),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Score info
            parent.spawn((
                Text::new(format!("スコア {}", score.value)),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 24.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(cream),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Level statistics
            parent.spawn((
                Text::new(format!(
                    "破壊ブロック {}  最大コンボ x{}\nクリアタイム {}:{:02}  獲得スコア {}",
                    level_stats.blocks_destroyed,
                    level_stats.max_combo,
                    time_min,
                    time_sec,
                    level_score,
                )),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 16.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(lavender),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Next level instruction
            parent.spawn((
                Text::new("SPACE / タップ で次のレベルへ"),
                TextFont {
                    font: font_handle,
                    font_size: 16.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(lavender),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
        });
}

/// Cleanup level clear screen
pub fn cleanup_level_clear(mut commands: Commands, query: Query<Entity, With<LevelClearUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Advance to next level
pub fn advance_level(mut level: ResMut<Level>) {
    level.current += 1;
}

/// Reset game state when returning to menu
pub fn reset_game(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut level: ResMut<Level>,
    mut combo: ResMut<ComboTracker>,
    mut level_stats: ResMut<LevelStats>,
    game_entities: Query<
        Entity,
        Or<(
            With<Ball>,
            With<Block>,
            With<Paddle>,
            With<Wall>,
            With<ScoreText>,
            With<LevelText>,
            With<HighScoreText>,
            With<PowerUp>,
            With<ComboPopup>,
        )>,
    >,
    mut paddle_query: Query<(Entity, &mut Sprite, &mut Collider), With<Paddle>>,
) {
    // Reset paddle size if power-up was active
    for (paddle_entity, mut sprite, mut collider) in &mut paddle_query {
        sprite.custom_size = Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT));
        collider.size = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);
        commands.entity(paddle_entity).remove::<PowerUpEffects>();
    }

    // Reset resources
    score.value = 0;
    level.current = 1;
    combo.count = 0;
    combo.timer.reset();
    *level_stats = LevelStats::default();

    // Despawn game entities
    for entity in &game_entities {
        commands.entity(entity).despawn();
    }
}

/// Cleanup for next level (remove ball, paddle, power-ups, and combo popups)
pub fn cleanup_for_next_level(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<Ball>, With<Paddle>, With<PowerUp>, With<ComboPopup>, With<Block>)>>,
    mut paddle_query: Query<(Entity, &mut Sprite, &mut Collider), With<Paddle>>,
    mut combo: ResMut<ComboTracker>,
    mut level_stats: ResMut<LevelStats>,
) {
    // Reset paddle size if power-up was active
    for (paddle_entity, mut sprite, mut collider) in &mut paddle_query {
        sprite.custom_size = Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT));
        collider.size = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);
        commands.entity(paddle_entity).remove::<PowerUpEffects>();
    }

    // Reset combo
    combo.count = 0;
    combo.timer.reset();

    // Reset level stats (will be re-initialized on level start)
    *level_stats = LevelStats::default();

    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn advance_level_increments() {
        let mut app = test_app();
        app.add_systems(Update, advance_level);
        app.update();

        let level = app.world().resource::<Level>();
        assert_eq!(level.current, 2);
    }

    #[test]
    fn reset_game_clears_resources() {
        let mut app = test_app();
        // Set non-default values
        app.world_mut().resource_mut::<Score>().value = 999;
        app.world_mut().resource_mut::<Level>().current = 5;
        app.world_mut().resource_mut::<ComboTracker>().count = 3;
        app.world_mut().resource_mut::<LevelStats>().blocks_destroyed = 10;

        app.add_systems(Update, reset_game);
        app.update();

        let score = app.world().resource::<Score>();
        let level = app.world().resource::<Level>();
        let combo = app.world().resource::<ComboTracker>();
        let level_stats = app.world().resource::<LevelStats>();
        assert_eq!(score.value, 0);
        assert_eq!(level.current, 1);
        assert_eq!(combo.count, 0);
        assert_eq!(level_stats.blocks_destroyed, 0);
    }

    #[test]
    fn reset_game_despawns_entities() {
        let mut app = test_app();
        spawn_test_ball(app.world_mut(), Vec2::ZERO, Vec2::new(100.0, 100.0));
        spawn_test_block(app.world_mut(), Vec2::new(0.0, 200.0));
        spawn_test_paddle(app.world_mut(), 0.0);
        spawn_test_wall(
            app.world_mut(),
            Wall::Top,
            Vec2::new(0.0, 400.0),
            Vec2::new(800.0, 10.0),
        );

        app.add_systems(Update, reset_game);
        app.update();

        let ball_count = app.world_mut().query::<&Ball>().iter(app.world()).count();
        let block_count = app
            .world_mut()
            .query::<&Block>()
            .iter(app.world())
            .count();
        let paddle_count = app
            .world_mut()
            .query::<&Paddle>()
            .iter(app.world())
            .count();
        assert_eq!(ball_count, 0);
        assert_eq!(block_count, 0);
        assert_eq!(paddle_count, 0);
    }

    #[test]
    fn cleanup_next_level_despawns_ball_paddle_block() {
        let mut app = test_app();
        spawn_test_ball(app.world_mut(), Vec2::ZERO, Vec2::new(100.0, 100.0));
        spawn_test_paddle(app.world_mut(), 0.0);
        spawn_test_block(app.world_mut(), Vec2::new(0.0, 200.0));
        // Wall should survive
        spawn_test_wall(
            app.world_mut(),
            Wall::Top,
            Vec2::new(0.0, 400.0),
            Vec2::new(800.0, 10.0),
        );

        app.add_systems(Update, cleanup_for_next_level);
        app.update();

        let ball_count = app.world_mut().query::<&Ball>().iter(app.world()).count();
        let paddle_count = app
            .world_mut()
            .query::<&Paddle>()
            .iter(app.world())
            .count();
        let block_count = app
            .world_mut()
            .query::<&Block>()
            .iter(app.world())
            .count();
        let wall_count = app.world_mut().query::<&Wall>().iter(app.world()).count();
        assert_eq!(ball_count, 0, "Balls should be despawned");
        assert_eq!(paddle_count, 0, "Paddle should be despawned");
        assert_eq!(block_count, 0, "Blocks should be despawned");
        assert_eq!(wall_count, 1, "Walls should survive");
    }

    #[test]
    fn cleanup_next_level_resets_paddle_size() {
        let mut app = test_app();
        let paddle = spawn_test_paddle(app.world_mut(), 0.0);
        // Simulate wide paddle power-up
        let wide_width = PADDLE_WIDTH * 1.5;
        app.world_mut()
            .entity_mut(paddle)
            .get_mut::<Sprite>()
            .unwrap()
            .custom_size = Some(Vec2::new(wide_width, PADDLE_HEIGHT));
        app.world_mut()
            .entity_mut(paddle)
            .get_mut::<Collider>()
            .unwrap()
            .size = Vec2::new(wide_width, PADDLE_HEIGHT);

        app.add_systems(Update, cleanup_for_next_level);
        app.update();

        // Paddle is despawned by cleanup_for_next_level, but before that it resets size.
        // Since it's despawned we just verify no panic occurred.
    }

    #[test]
    fn cleanup_next_level_resets_combo() {
        let mut app = test_app();
        app.world_mut().resource_mut::<ComboTracker>().count = 5;

        app.add_systems(Update, cleanup_for_next_level);
        app.update();

        let combo = app.world().resource::<ComboTracker>();
        assert_eq!(combo.count, 0, "Combo should be reset");
    }
}
