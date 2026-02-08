use bevy::input::touch::Touches;
use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::TestPlayMode;
use crate::states::GameState;

/// Handle paddle movement input
pub fn paddle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Paddle>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Ok(mut paddle_transform) = query.get_single_mut() else {
        return;
    };

    let half_paddle = PADDLE_WIDTH / 2.0;
    let limit = WINDOW_WIDTH / 2.0 - WALL_THICKNESS - half_paddle;

    // Touch input: move paddle directly to touch X position
    if let Some(screen_pos) = touches.first_pressed_position() {
        if let Ok((camera, cam_transform)) = camera_query.get_single() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(cam_transform, screen_pos) {
                paddle_transform.translation.x = world_pos.x.clamp(-limit, limit);
                return;
            }
        }
    }

    // Keyboard input
    let mut direction = 0.0;

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    let new_x = paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();
    paddle_transform.translation.x = new_x.clamp(-limit, limit);
}

/// Handle game start input (Space or tap to start, S for settings, E for editor)
pub fn start_game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
    settings_btn: Query<&Interaction, (With<crate::components::SettingsButton>, Without<EditorButton>)>,
    editor_btn: Query<&Interaction, (With<EditorButton>, Without<crate::components::SettingsButton>)>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        next_state.set(GameState::Settings);
        return;
    }
    if keyboard.just_pressed(KeyCode::KeyE) {
        next_state.set(GameState::Editor);
        return;
    }
    // Check if settings button was tapped
    for interaction in &settings_btn {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Settings);
            return;
        }
    }
    // Check if editor button was tapped
    for interaction in &editor_btn {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Editor);
            return;
        }
    }
    if keyboard.just_pressed(KeyCode::Space) || touches.any_just_pressed() {
        next_state.set(GameState::Countdown);
    }
}

/// Handle pause input (Playing state only — resume is handled by pause_overlay_input)
pub fn pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    pause_btn: Query<&Interaction, With<PauseButton>>,
) {
    let btn_pressed = pause_btn.iter().any(|i| *i == Interaction::Pressed);
    if keyboard.just_pressed(KeyCode::Escape) || btn_pressed {
        next_state.set(GameState::Paused);
    }
}

/// Handle restart input after game over
pub fn restart_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
    test_play: Option<Res<TestPlayMode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || touches.any_just_pressed() {
        if test_play.is_some() {
            next_state.set(GameState::Editor);
        } else {
            next_state.set(GameState::Menu);
        }
    }
}

/// Handle next level input
pub fn next_level_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
    test_play: Option<Res<TestPlayMode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || touches.any_just_pressed() {
        if test_play.is_some() {
            next_state.set(GameState::Editor);
        } else {
            next_state.set(GameState::Countdown);
        }
    }
}
