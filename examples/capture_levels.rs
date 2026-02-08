//! Captures screenshots of each level's initial state.
//!
//! Usage:
//!   cargo run --example capture_levels
//!
//! Outputs: screenshots/level_1.png .. screenshots/level_5.png

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::texture::ImagePlugin;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

// Import game modules
use breakout::components::*;
use breakout::constants::*;
use breakout::resources::*;
use breakout::states::GameState;
use breakout::systems::*;

/// Orchestration state for level capture
#[derive(Resource)]
struct CaptureState {
    current_level: u32,
    max_level: u32,
    frames_waited: u32,
    frames_to_wait: u32,
    phase: CapturePhase,
}

#[derive(PartialEq)]
enum CapturePhase {
    WaitingToCapture,
    WaitingForSave,
    Done,
}

impl Default for CaptureState {
    fn default() -> Self {
        Self {
            current_level: 1,
            max_level: 5,
            frames_waited: 0,
            frames_to_wait: 10,
            phase: CapturePhase::WaitingToCapture,
        }
    }
}

fn main() {
    // Create screenshots directory
    std::fs::create_dir_all("screenshots").expect("Failed to create screenshots directory");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Level Screenshot Capture".to_string(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
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
        // Core resources and startup (reuse game's CorePlugin setup)
        .insert_resource(ClearColor(Color::srgb(0.07, 0.07, 0.16)))
        .init_resource::<Score>()
        .init_resource::<Level>()
        .init_resource::<GameSounds>()
        .init_resource::<ScreenShake>()
        .init_resource::<ComboTracker>()
        .insert_resource(HighScores::default())
        .init_resource::<LevelStats>()
        .init_resource::<CaptureState>()
        .init_state::<GameState>()
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup_camera)
        // OnEnter(Playing) systems from GameplayPlugin
        .add_systems(
            OnEnter(GameState::Playing),
            (spawn_paddle, spawn_ball, spawn_blocks, spawn_walls, spawn_ui)
                .run_if(not(any_with_component::<Block>)),
        )
        // Keep level text updated so HUD shows correct level number
        .add_systems(Update, update_level_text)
        // Orchestration system runs every frame
        .add_systems(Update, orchestrate_capture)
        .run();
}

/// Orchestration system: waits for rendering, captures screenshot, advances level
fn orchestrate_capture(
    mut commands: Commands,
    mut capture: ResMut<CaptureState>,
    mut level: ResMut<Level>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
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
    node_entities: Query<
        Entity,
        (
            With<Node>,
            Without<ScoreText>,
            Without<LevelText>,
            Without<HighScoreText>,
        ),
    >,
    mut exit: EventWriter<AppExit>,
) {
    // All done — wait a few frames for the last screenshot to be written, then exit
    if capture.phase == CapturePhase::Done {
        capture.frames_waited += 1;
        if capture.frames_waited >= 5 {
            info!("All {} levels captured. Exiting.", capture.max_level);
            exit.send(AppExit::Success);
        }
        return;
    }

    // Phase 1: Transition to Playing state for current level
    if *state.get() == GameState::Menu {
        level.current = capture.current_level;
        next_state.set(GameState::Playing);
        return;
    }

    // Only proceed in Playing state
    if *state.get() != GameState::Playing {
        return;
    }

    // Phase 2: Wait for rendering to stabilize, then capture
    if capture.phase == CapturePhase::WaitingToCapture {
        capture.frames_waited += 1;
        if capture.frames_waited < capture.frames_to_wait {
            return;
        }

        // Take screenshot
        let path = format!("screenshots/level_{}.png", capture.current_level);
        info!("Capturing screenshot: {}", path);
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
        capture.phase = CapturePhase::WaitingForSave;
        capture.frames_waited = 0;
        return;
    }

    // Phase 3: Wait a few frames for the screenshot file to be written
    if capture.phase == CapturePhase::WaitingForSave {
        capture.frames_waited += 1;
        if capture.frames_waited < 3 {
            return;
        }

        // Check if all levels are done
        if capture.current_level >= capture.max_level {
            capture.phase = CapturePhase::Done;
            capture.frames_waited = 0;
            return;
        }

        // Despawn all game entities for clean slate
        for entity in &game_entities {
            commands.entity(entity).despawn_recursive();
        }
        // Despawn UI nodes (HUD bar background etc.)
        for entity in &node_entities {
            commands.entity(entity).despawn_recursive();
        }

        // Advance to next level
        capture.current_level += 1;
        capture.frames_waited = 0;
        capture.phase = CapturePhase::WaitingToCapture;

        // Transition: Playing -> Menu -> Playing to re-trigger OnEnter(Playing)
        next_state.set(GameState::Menu);
    }
}
