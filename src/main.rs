use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;

mod components;
mod constants;
mod resources;
mod states;
mod systems;

use components::{Block, BgmMusic, Paddle};
use constants::*;
use resources::*;
use states::GameState;
use systems::audio::CollisionEvent;
use systems::*;

fn main() {
    App::new()
        // Window configuration
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ブロック崩し".to_string(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        canvas: Some("#bevy_canvas".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // Background color (retro dark indigo)
        .insert_resource(ClearColor(Color::srgb(0.07, 0.07, 0.16)))
        // Initialize resources
        .init_resource::<Score>()
        .init_resource::<Level>()
        .init_resource::<GameSounds>()
        .init_resource::<ScreenShake>()
        .init_resource::<ComboTracker>()
        // Initialize state
        .init_state::<GameState>()
        // Register events
        .add_event::<CollisionEvent>()
        // Startup systems
        .add_systems(Startup, (setup_camera, load_sounds))
        // Menu state
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(Update, start_game_input.run_if(in_state(GameState::Menu)))
        // Playing state - enter
        .add_systems(
            OnEnter(GameState::Playing),
            (spawn_paddle, spawn_ball, spawn_blocks, spawn_walls, spawn_ui)
                .run_if(not(any_with_component::<Block>)),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            (spawn_paddle, spawn_ball)
                .run_if(any_with_component::<Block>.and(not(any_with_component::<Paddle>))),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            start_bgm.run_if(not(any_with_component::<BgmMusic>)),
        )
        // UI scale (runs always)
        .add_systems(Update, update_ui_scale)
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
                powerup_movement,
                paddle_powerup_collision,
                update_powerup_effects,
                update_combo_timer,
                spawn_combo_popup,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Particle, screen shake, and combo popup effects (run always so they finish even after state change)
        .add_systems(Update, (update_particles, apply_screen_shake, update_combo_popup))
        // Sound system runs in all states to catch game over / level clear events
        .add_systems(Update, play_collision_sounds)
        // Paused state
        .add_systems(OnEnter(GameState::Paused), pause_bgm)
        .add_systems(OnExit(GameState::Paused), resume_bgm)
        .add_systems(Update, pause_input.run_if(in_state(GameState::Paused)))
        // Game Over state
        .add_systems(OnEnter(GameState::GameOver), (setup_game_over, stop_bgm))
        .add_systems(OnExit(GameState::GameOver), (cleanup_game_over, reset_game))
        .add_systems(
            Update,
            restart_input.run_if(in_state(GameState::GameOver)),
        )
        // Level Clear state
        .add_systems(OnEnter(GameState::LevelClear), (setup_level_clear, stop_bgm))
        .add_systems(
            OnExit(GameState::LevelClear),
            (cleanup_level_clear, cleanup_for_next_level, advance_level),
        )
        .add_systems(
            Update,
            next_level_input.run_if(in_state(GameState::LevelClear)),
        )
        .run();
}
