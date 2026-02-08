use bevy::prelude::*;

use crate::components::{Block, BgmMusic, CountdownDisplay, Paddle};
use crate::resources::*;
use crate::states::GameState;
use crate::systems::audio::CollisionEvent;
use crate::systems::*;
use crate::systems::effects::TrailTimer;

/// Core plugin: resources, events, startup systems, and always-running systems
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::srgb(0.07, 0.07, 0.16)))
            .init_resource::<Score>()
            .init_resource::<Level>()
            .init_resource::<GameSounds>()
            .init_resource::<ScreenShake>()
            .init_resource::<ComboTracker>()
            .insert_resource(HighScores::load())
            .init_resource::<LevelStats>()
            .init_resource::<TrailTimer>()
            .insert_resource(AudioSettings::load())
            .init_state::<GameState>()
            .add_event::<CollisionEvent>()
            .add_systems(Startup, (setup_camera, load_sounds))
            .add_systems(Update, update_ui_scale)
            .add_systems(Update, (update_particles, apply_screen_shake, update_combo_popup))
            .add_systems(Update, play_collision_sounds);
    }
}

/// Menu plugin: menu screen systems
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(Update, start_game_input.run_if(in_state(GameState::Menu)));
    }
}

/// Gameplay plugin: playing and paused state systems
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Countdown state - enter (fresh level: spawn field + countdown)
            // Normal game: spawn level-based blocks
            .add_systems(
                OnEnter(GameState::Countdown),
                (spawn_paddle, spawn_ball, spawn_blocks, spawn_walls, spawn_ui, record_level_start_score)
                    .run_if(not(any_with_component::<Block>).and(not(resource_exists::<TestPlayMode>))),
            )
            // Test play: spawn blocks from editor grid
            .add_systems(
                OnEnter(GameState::Countdown),
                (spawn_paddle, spawn_ball, spawn_blocks_from_editor, spawn_walls, spawn_ui, record_level_start_score)
                    .run_if(not(any_with_component::<Block>).and(resource_exists::<TestPlayMode>)),
            )
            .add_systems(
                OnEnter(GameState::Countdown),
                spawn_countdown.run_if(not(any_with_component::<CountdownDisplay>)),
            )
            // Countdown state - update (paddle movable, countdown ticking)
            .add_systems(
                Update,
                (paddle_input, update_countdown)
                    .run_if(in_state(GameState::Countdown)),
            )
            // Countdown state - exit
            .add_systems(OnExit(GameState::Countdown), cleanup_countdown)
            // Playing state - enter (start BGM after countdown)
            .add_systems(
                OnEnter(GameState::Playing),
                start_bgm.run_if(not(any_with_component::<BgmMusic>)),
            )
            // Playing state - enter (continue after pause or life lost)
            .add_systems(
                OnEnter(GameState::Playing),
                (spawn_paddle, spawn_ball)
                    .run_if(any_with_component::<Block>.and(not(any_with_component::<Paddle>))),
            )
            // Playing state - update
            .add_systems(
                Update,
                (
                    paddle_input,
                    pause_input,
                    ball_movement,
                    ball_paddle_collision,
                    ball_wall_collision,
                    ball_block_collision,
                    check_level_clear,
                    update_score_text,
                    update_level_text,
                    update_high_score_text,
                    update_level_time,
                    powerup_movement,
                    paddle_powerup_collision,
                    update_powerup_effects,
                    update_combo_timer,
                    spawn_combo_popup,
                    spawn_ball_trail,
                    update_fireball_visual,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            // Paused state
            .add_systems(OnEnter(GameState::Paused), (pause_bgm, setup_pause))
            .add_systems(OnExit(GameState::Paused), (resume_bgm, cleanup_pause))
            .add_systems(Update, pause_overlay_input.run_if(in_state(GameState::Paused)));
    }
}

/// Game over plugin: game over screen systems
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), (setup_game_over, stop_bgm))
            .add_systems(OnExit(GameState::GameOver), (cleanup_game_over, reset_game))
            .add_systems(Update, (
                restart_input,
                update_new_record_flash,
                update_rank_marker,
            ).run_if(in_state(GameState::GameOver)));
    }
}

/// Settings plugin: settings screen systems
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Settings), setup_settings)
            .add_systems(OnExit(GameState::Settings), cleanup_settings)
            .add_systems(Update, settings_input.run_if(in_state(GameState::Settings)));
    }
}

/// Level clear plugin: level clear screen systems
pub struct LevelClearPlugin;

impl Plugin for LevelClearPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::LevelClear), (setup_level_clear, stop_bgm))
            .add_systems(
                OnExit(GameState::LevelClear),
                (cleanup_level_clear, cleanup_for_next_level, advance_level),
            )
            .add_systems(
                Update,
                next_level_input.run_if(in_state(GameState::LevelClear)),
            );
    }
}

/// Editor plugin: stage editor + test play systems
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<EditorState>()
            .add_systems(Startup, load_stage_from_url)
            // Editor state
            .add_systems(OnEnter(GameState::Editor), setup_editor)
            .add_systems(OnExit(GameState::Editor), cleanup_editor)
            .add_systems(
                Update,
                (
                    editor_grid_input,
                    editor_tool_select,
                    editor_share,
                    update_share_feedback,
                    editor_test_play,
                    editor_back_to_menu,
                )
                    .run_if(in_state(GameState::Editor)),
            )
            // TestPlay state: spawn editor blocks + countdown, then go to Countdown
            .add_systems(
                OnEnter(GameState::TestPlay),
                enter_test_play,
            );
    }
}

/// System: enter test play mode — insert TestPlayMode marker, spawn editor blocks, transition to Countdown
fn enter_test_play(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.insert_resource(TestPlayMode);
    next_state.set(GameState::Countdown);
}
