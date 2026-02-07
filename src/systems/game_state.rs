use bevy::prelude::*;
use bevy::text::FontSmoothing;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

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
pub fn setup_game_over(mut commands: Commands, score: Res<Score>, level: Res<Level>, asset_server: Res<AssetServer>) {
    let salmon = Color::srgb(0.92, 0.44, 0.44);
    let cream = Color::srgb(0.95, 0.85, 0.65);
    let lavender = Color::srgb(0.55, 0.50, 0.65);
    let font_handle: Handle<Font> = asset_server.load(GAME_FONT_PATH);

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
pub fn setup_level_clear(mut commands: Commands, score: Res<Score>, level: Res<Level>, asset_server: Res<AssetServer>) {
    let soft_green = Color::srgb(0.40, 0.80, 0.52);
    let cream = Color::srgb(0.95, 0.85, 0.65);
    let lavender = Color::srgb(0.55, 0.50, 0.65);
    let font_handle: Handle<Font> = asset_server.load(GAME_FONT_PATH);

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
    game_entities: Query<
        Entity,
        Or<(
            With<Ball>,
            With<Block>,
            With<Paddle>,
            With<Wall>,
            With<ScoreText>,
            With<LevelText>,
            With<PowerUp>,
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

    // Despawn game entities
    for entity in &game_entities {
        commands.entity(entity).despawn();
    }
}

/// Cleanup for next level (remove ball, paddle, and power-ups)
pub fn cleanup_for_next_level(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<Ball>, With<Paddle>, With<PowerUp>)>>,
    mut paddle_query: Query<(Entity, &mut Sprite, &mut Collider), With<Paddle>>,
) {
    // Reset paddle size if power-up was active
    for (paddle_entity, mut sprite, mut collider) in &mut paddle_query {
        sprite.custom_size = Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT));
        collider.size = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);
        commands.entity(paddle_entity).remove::<PowerUpEffects>();
    }

    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
