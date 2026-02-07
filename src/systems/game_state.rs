use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

/// Setup menu screen
pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cyan = Color::srgb(0.0, 0.9, 1.0);
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
            BackgroundColor(Color::srgba(0.02, 0.02, 0.08, 0.85)),
            MenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("ブロック崩し"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(cyan),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Start instruction
            parent.spawn((
                Text::new("SPACE でスタート"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Controls
            parent.spawn((
                Text::new("← → / A D  パドル操作\nESC  ポーズ"),
                TextFont {
                    font: font_handle,
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
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
    let magenta = Color::srgb(1.0, 0.0, 0.6);
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
            BackgroundColor(Color::srgba(0.02, 0.02, 0.08, 0.85)),
            GameOverUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("ゲームオーバー"),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 50.0,
                    ..default()
                },
                TextColor(magenta),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Score info
            parent.spawn((
                Text::new(format!("レベル {}\n最終スコア {}", level.current, score.value)),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Restart instruction
            parent.spawn((
                Text::new("SPACE でリトライ"),
                TextFont {
                    font: font_handle,
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
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
    let neon_green = Color::srgb(0.0, 1.0, 0.6);
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
            BackgroundColor(Color::srgba(0.02, 0.02, 0.08, 0.85)),
            LevelClearUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new(format!("レベル {} クリア！", level.current)),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 50.0,
                    ..default()
                },
                TextColor(neon_green),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Score info
            parent.spawn((
                Text::new(format!("スコア {}", score.value)),
                TextFont {
                    font: font_handle.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Next level instruction
            parent.spawn((
                Text::new("SPACE で次のレベルへ"),
                TextFont {
                    font: font_handle,
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
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
        )>,
    >,
) {
    // Reset resources
    score.value = 0;
    level.current = 1;

    // Despawn game entities
    for entity in &game_entities {
        commands.entity(entity).despawn();
    }
}

/// Cleanup for next level (remove ball and paddle only)
pub fn cleanup_for_next_level(
    mut commands: Commands,
    entities: Query<Entity, Or<(With<Ball>, With<Paddle>)>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}
