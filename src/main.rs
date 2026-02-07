use bevy::prelude::*;

mod components;
mod constants;
mod resources;
mod states;
mod systems;

use components::Block;
use constants::*;
use resources::*;
use states::GameState;
use systems::audio::CollisionEvent;
use systems::*;

fn main() {
    App::new()
        // Window configuration
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ブロック崩し".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        // Background color (dark navy)
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.12)))
        // Initialize resources
        .init_resource::<Score>()
        .init_resource::<Level>()
        .init_resource::<GameSounds>()
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
            (spawn_paddle, spawn_ball).run_if(any_with_component::<Block>),
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
                play_collision_sounds,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Paused state
        .add_systems(Update, pause_input.run_if(in_state(GameState::Paused)))
        // Game Over state
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(OnExit(GameState::GameOver), (cleanup_game_over, reset_game))
        .add_systems(
            Update,
            restart_input.run_if(in_state(GameState::GameOver)),
        )
        // Level Clear state
        .add_systems(OnEnter(GameState::LevelClear), setup_level_clear)
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
