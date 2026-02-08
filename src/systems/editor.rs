use bevy::prelude::*;
use bevy::text::FontSmoothing;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::states::GameState;
use crate::systems::setup::{block_type_color, grid_x, grid_y, spawn_block};

/// Color for an empty editor grid cell
const EMPTY_CELL_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.08);
/// Color for the selected tool highlight border
const TOOL_SELECTED_COLOR: Color = Color::srgb(1.0, 0.85, 0.20);

/// Get the display color for a block type in the editor (use row=0 for Normal)
fn editor_block_color(bt: &BlockType) -> Color {
    block_type_color(bt, 0)
}

/// Setup the editor UI
pub fn setup_editor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    editor: Res<EditorState>,
) {
    let warm_white = Color::srgb(1.0, 0.96, 0.88);
    let cream = Color::srgb(0.95, 0.85, 0.65);
    let lavender = Color::srgb(0.55, 0.50, 0.65);
    let font: Handle<Font> = asset_server.load(GAME_FONT_PATH);

    // Root container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.12, 0.95)),
            EditorUI,
        ))
        .with_children(|root| {
            // Title
            root.spawn((
                Text::new("ステージエディタ"),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(warm_white),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Main area: tool palette (left) + grid (right)
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexStart,
                column_gap: Val::Px(12.0),
                ..default()
            })
            .with_children(|main_area| {
                // Tool palette
                main_area
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(6.0),
                        padding: UiRect::all(Val::Px(4.0)),
                        ..default()
                    })
                    .with_children(|palette| {
                        // Palette label
                        palette.spawn((
                            Text::new("TOOL"),
                            TextFont {
                                font: font.clone(),
                                font_size: 16.0,
                                font_smoothing: FontSmoothing::None,
                            },
                            TextColor(lavender),
                        ));

                        let tools: Vec<(Option<BlockType>, &str, &str, Color)> = vec![
                            (Some(BlockType::Normal), "N", "通常", editor_block_color(&BlockType::Normal)),
                            (Some(BlockType::Durable { hits_remaining: 2 }), "D", "耐久", editor_block_color(&BlockType::Durable { hits_remaining: 2 })),
                            (Some(BlockType::Steel), "S", "鉄", editor_block_color(&BlockType::Steel)),
                            (Some(BlockType::Explosive), "E", "爆発", editor_block_color(&BlockType::Explosive)),
                            (None, "×", "消去", Color::srgb(0.3, 0.3, 0.3)),
                        ];

                        for (tool_type, icon, desc, color) in tools {
                            let is_selected = editor.selected_tool == tool_type;
                            palette
                                .spawn((
                                    Button,
                                    Node {
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        column_gap: Val::Px(6.0),
                                        padding: UiRect::axes(Val::Px(4.0), Val::Px(2.0)),
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    BackgroundColor(Color::NONE),
                                    BorderColor(if is_selected { TOOL_SELECTED_COLOR } else { Color::NONE }),
                                    ToolButton(tool_type),
                                ))
                                .with_children(|btn| {
                                    // Color swatch
                                    btn.spawn((
                                        Node {
                                            width: Val::Px(28.0),
                                            height: Val::Px(20.0),
                                            ..default()
                                        },
                                        BackgroundColor(color),
                                    ));
                                    // Description label
                                    btn.spawn((
                                        Text::new(format!("{} {}", icon, desc)),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 16.0,
                                            font_smoothing: FontSmoothing::None,
                                        },
                                        TextColor(if is_selected { TOOL_SELECTED_COLOR } else { cream }),
                                    ));
                                });
                        }
                    });

                // Grid container
                main_area
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(EDITOR_CELL_GAP),
                        ..default()
                    })
                    .with_children(|grid_container| {
                        for row in 0..EDITOR_ROWS {
                            grid_container
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(EDITOR_CELL_GAP),
                                    ..default()
                                })
                                .with_children(|row_node| {
                                    for col in 0..EDITOR_COLS {
                                        let cell_color = match editor.grid[row][col] {
                                            Some(ref bt) => editor_block_color(bt),
                                            None => EMPTY_CELL_COLOR,
                                        };

                                        row_node.spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(EDITOR_CELL_SIZE),
                                                height: Val::Px(36.0),
                                                ..default()
                                            },
                                            BackgroundColor(cell_color),
                                            GridCell { row, col },
                                        ));
                                    }
                                });
                        }
                    });
            });

            // Bottom buttons row
            root.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            })
            .with_children(|buttons| {
                // Test play button
                spawn_editor_button(buttons, &font, "テストプレイ", cream, TestPlayButton);
                // Share button
                spawn_editor_button(buttons, &font, "共有", cream, ShareButton);
                // Back to menu button
                spawn_editor_button(buttons, &font, "メニュー", cream, EditorMenuButton);
            });

            // Share feedback text (initially invisible)
            root.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(Color::srgb(0.40, 1.0, 0.50)),
                ShareFeedback {
                    timer: Timer::from_seconds(2.0, TimerMode::Once),
                },
            ));
        });
}

fn spawn_editor_button(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    label: &str,
    text_color: Color,
    marker: impl Component,
) {
    parent
        .spawn((
            Button,
            Node {
                padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.08)),
            BorderColor(Color::srgba(1.0, 1.0, 1.0, 0.2)),
            marker,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(text_color),
            ));
        });
}

/// Handle grid cell clicks to place/remove blocks
pub fn editor_grid_input(
    mut editor: ResMut<EditorState>,
    mut cells: Query<(&Interaction, &GridCell, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, cell, mut bg) in &mut cells {
        if *interaction == Interaction::Pressed {
            editor.grid[cell.row][cell.col] = editor.selected_tool;
            *bg = BackgroundColor(match editor.selected_tool {
                Some(ref bt) => editor_block_color(bt),
                None => EMPTY_CELL_COLOR,
            });
        }
    }
}

/// Handle tool palette button clicks
pub fn editor_tool_select(
    mut editor: ResMut<EditorState>,
    mut tools: Query<(&Interaction, &ToolButton, &mut BorderColor)>,
) {
    // First pass: detect if any tool was pressed
    let mut new_selection = None;
    for (interaction, tool, _) in &tools {
        if *interaction == Interaction::Pressed {
            new_selection = Some(tool.0);
        }
    }

    // Second pass: update all borders if selection changed
    if let Some(sel) = new_selection {
        editor.selected_tool = sel;
        for (_, tool, mut border) in &mut tools {
            if tool.0 == sel {
                *border = BorderColor(TOOL_SELECTED_COLOR);
            } else {
                *border = BorderColor(Color::NONE);
            }
        }
    }
}

/// Handle share button press
pub fn editor_share(
    editor: Res<EditorState>,
    share_btn: Query<&Interaction, (With<ShareButton>, Changed<Interaction>)>,
    mut feedback: Query<(&mut Text, &mut ShareFeedback)>,
) {
    for interaction in &share_btn {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let json = serde_json::to_string(&editor.grid).unwrap_or_default();
        let encoded = base64_encode(&json);

        #[cfg(target_arch = "wasm32")]
        {
            copy_to_clipboard_wasm(&encoded);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            bevy::log::info!("Stage URL param: ?stage={}", encoded);
        }

        // Show feedback
        if let Ok((mut text, mut fb)) = feedback.get_single_mut() {
            **text = "URLをコピーしました！".to_string();
            fb.timer.reset();
        }
    }
}

/// Fade out the share feedback text
pub fn update_share_feedback(
    time: Res<Time>,
    mut feedback: Query<(&mut Text, &mut ShareFeedback)>,
) {
    for (mut text, mut fb) in &mut feedback {
        fb.timer.tick(time.delta());
        if fb.timer.just_finished() {
            **text = "".to_string();
        }
    }
}

/// Handle test play button press
pub fn editor_test_play(
    test_btn: Query<&Interaction, (With<TestPlayButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &test_btn {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::TestPlay);
        }
    }
}

/// Handle back to menu button press
pub fn editor_back_to_menu(
    menu_btn: Query<&Interaction, (With<EditorMenuButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &menu_btn {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Menu);
        }
    }
}

/// Cleanup all editor UI entities
pub fn cleanup_editor(mut commands: Commands, query: Query<Entity, With<EditorUI>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Spawn blocks from editor grid data
pub fn spawn_blocks_from_editor(mut commands: Commands, editor: Res<EditorState>) {
    for row in 0..EDITOR_ROWS {
        for col in 0..EDITOR_COLS {
            if let Some(block_type) = editor.grid[row][col] {
                spawn_block(&mut commands, grid_x(col), grid_y(row), block_type, row);
            }
        }
    }
}

/// Check if editor grid has any non-steel clearable blocks
#[cfg(test)]
fn editor_has_clearable_blocks(editor: &EditorState) -> bool {
    editor.grid.iter().flatten().any(|cell| {
        matches!(cell, Some(bt) if !matches!(bt, BlockType::Steel))
    })
}

/// Load stage from URL parameter on startup (WASM only)
pub fn load_stage_from_url(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(stage_data) = get_url_stage_param() {
        if let Some(grid) = decode_stage(&stage_data) {
            commands.insert_resource(EditorState {
                selected_tool: Some(BlockType::Normal),
                grid,
            });
            next_state.set(GameState::Editor);
        }
    }
}

/// Decode a base64-encoded stage string into a grid
fn decode_stage(encoded: &str) -> Option<[[Option<BlockType>; 10]; 7]> {
    let json = base64_decode(encoded)?;
    serde_json::from_str(&json).ok()
}

/// Simple Base64 encoding (URL-safe)
fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };

        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        }
    }

    result
}

/// Simple Base64 decoding (URL-safe)
fn base64_decode(input: &str) -> Option<String> {
    fn char_to_val(c: u8) -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some((c - b'A') as u32),
            b'a'..=b'z' => Some((c - b'a' + 26) as u32),
            b'0'..=b'9' => Some((c - b'0' + 52) as u32),
            b'-' => Some(62),
            b'_' => Some(63),
            _ => None,
        }
    }

    let bytes = input.as_bytes();
    let mut result = Vec::new();

    let mut i = 0;
    while i < bytes.len() {
        let remaining = bytes.len() - i;

        let v0 = char_to_val(bytes[i])?;
        let v1 = if i + 1 < bytes.len() { char_to_val(bytes[i + 1])? } else { 0 };
        let v2 = if i + 2 < bytes.len() { char_to_val(bytes[i + 2])? } else { 0 };
        let v3 = if i + 3 < bytes.len() { char_to_val(bytes[i + 3])? } else { 0 };

        let triple = (v0 << 18) | (v1 << 12) | (v2 << 6) | v3;

        result.push(((triple >> 16) & 0xFF) as u8);
        if remaining > 2 {
            result.push(((triple >> 8) & 0xFF) as u8);
        }
        if remaining > 3 {
            result.push((triple & 0xFF) as u8);
        }

        i += 4.min(remaining);
        // If we consumed less than 4 chars, break
        if remaining < 4 {
            break;
        }
    }

    String::from_utf8(result).ok()
}

/// Get stage parameter from URL (WASM)
#[cfg(target_arch = "wasm32")]
fn get_url_stage_param() -> Option<String> {
    let window = web_sys::window()?;
    let search = window.location().search().ok()?;
    let prefix = "?stage=";
    if search.starts_with(prefix) {
        Some(search[prefix.len()..].to_string())
    } else if search.contains("&stage=") {
        let idx = search.find("&stage=")?;
        let rest = &search[idx + 7..];
        let end = rest.find('&').unwrap_or(rest.len());
        Some(rest[..end].to_string())
    } else if search.contains("?stage=") {
        let idx = search.find("?stage=")?;
        let rest = &search[idx + 7..];
        let end = rest.find('&').unwrap_or(rest.len());
        Some(rest[..end].to_string())
    } else {
        None
    }
}

/// Native: no URL stage param
#[cfg(not(target_arch = "wasm32"))]
fn get_url_stage_param() -> Option<String> {
    None
}

/// Copy URL to clipboard (WASM)
#[cfg(target_arch = "wasm32")]
fn copy_to_clipboard_wasm(encoded: &str) {
    let _ = (|| -> Option<()> {
        let window = web_sys::window()?;
        let location = window.location();
        let origin = location.origin().ok()?;
        let pathname = location.pathname().ok()?;
        let url = format!("{}{}?stage={}", origin, pathname, encoded);

        // Try clipboard API
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = clipboard.write_text(&url);

        // Also update URL bar
        let _ = window.history().ok()?.replace_state_with_url(
            &wasm_bindgen::JsValue::NULL,
            "",
            Some(&url),
        );

        Some(())
    })();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_roundtrip() {
        let input = r#"[[null,"Normal",null],[null,null,null]]"#;
        let encoded = base64_encode(input);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn base64_roundtrip_full_grid() {
        let grid: [[Option<BlockType>; 10]; 7] = [[None; 10]; 7];
        let json = serde_json::to_string(&grid).unwrap();
        let encoded = base64_encode(&json);
        let decoded = base64_decode(&encoded).unwrap();
        let grid2: [[Option<BlockType>; 10]; 7] = serde_json::from_str(&decoded).unwrap();
        assert_eq!(grid, grid2);
    }

    #[test]
    fn base64_roundtrip_with_blocks() {
        let mut grid: [[Option<BlockType>; 10]; 7] = [[None; 10]; 7];
        grid[0][0] = Some(BlockType::Normal);
        grid[1][5] = Some(BlockType::Durable { hits_remaining: 2 });
        grid[3][3] = Some(BlockType::Steel);
        grid[6][9] = Some(BlockType::Explosive);

        let json = serde_json::to_string(&grid).unwrap();
        let encoded = base64_encode(&json);
        let decoded = base64_decode(&encoded).unwrap();
        let grid2: [[Option<BlockType>; 10]; 7] = serde_json::from_str(&decoded).unwrap();
        assert_eq!(grid, grid2);
    }

    #[test]
    fn decode_stage_valid() {
        let grid: [[Option<BlockType>; 10]; 7] = [[None; 10]; 7];
        let json = serde_json::to_string(&grid).unwrap();
        let encoded = base64_encode(&json);
        let result = decode_stage(&encoded);
        assert!(result.is_some());
    }

    #[test]
    fn decode_stage_invalid() {
        let _result = decode_stage("not_valid_base64!@#");
        // Should not panic, may return None or Some with garbage
    }

    #[test]
    fn editor_has_clearable_blocks_empty() {
        let editor = EditorState::default();
        assert!(!editor_has_clearable_blocks(&editor));
    }

    #[test]
    fn editor_has_clearable_blocks_normal() {
        let mut editor = EditorState::default();
        editor.grid[0][0] = Some(BlockType::Normal);
        assert!(editor_has_clearable_blocks(&editor));
    }

    #[test]
    fn editor_has_clearable_blocks_steel_only() {
        let mut editor = EditorState::default();
        editor.grid[0][0] = Some(BlockType::Steel);
        assert!(!editor_has_clearable_blocks(&editor));
    }
}
