use bevy::audio::Volume;
use bevy::prelude::*;
use bevy::text::FontSmoothing;

use crate::components::{CountdownDisplay, CountdownText};
use crate::constants::*;
use crate::resources::{AudioSettings, GameSounds, GAME_FONT_PATH};
use crate::states::GameState;

const COUNTDOWN_START_SCALE: f32 = 1.8;

/// Spawn countdown UI: centered container with animated number text
pub fn spawn_countdown(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sounds: Res<GameSounds>,
    audio_settings: Res<AudioSettings>,
) {
    let font_handle: Handle<Font> = asset_server.load(GAME_FONT_PATH);
    let gold = Color::srgb(1.0, 0.85, 0.20);

    // Full-screen centering container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::bottom(Val::Percent(25.0)),
                ..default()
            },
            GlobalZIndex(20),
            CountdownDisplay {
                timer: Timer::from_seconds(COUNTDOWN_STEP_DURATION, TimerMode::Once),
                count: 3,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("3"),
                TextFont {
                    font: font_handle,
                    font_size: 96.0,
                    font_smoothing: FontSmoothing::None,
                },
                TextColor(gold),
                TextLayout::new_with_justify(JustifyText::Center),
                Transform::from_scale(Vec3::splat(COUNTDOWN_START_SCALE)),
                CountdownText,
            ));
        });

    // Play initial beep
    play_beep(&mut commands, &sounds, &audio_settings);
}

/// Animate scale, tick timer, advance 3→2→1→GO!→Playing
pub fn update_countdown(
    time: Res<Time>,
    mut commands: Commands,
    mut display_query: Query<(&mut CountdownDisplay, &Children)>,
    mut text_query: Query<(&mut Text, &mut TextColor, &mut Transform), With<CountdownText>>,
    mut next_state: ResMut<NextState<GameState>>,
    sounds: Res<GameSounds>,
    audio_settings: Res<AudioSettings>,
) {
    for (mut display, children) in &mut display_query {
        display.timer.tick(time.delta());

        // Find the child text entity
        let Some(&child) = children.iter().next() else {
            continue;
        };
        let Ok((mut text, mut text_color, mut transform)) = text_query.get_mut(child) else {
            continue;
        };

        // Animate scale: ease-out from COUNTDOWN_START_SCALE to 1.0
        let elapsed = display.timer.elapsed_secs();
        let anim_progress = (elapsed / COUNTDOWN_SCALE_ANIM_DURATION).min(1.0);
        let eased = 1.0 - (1.0 - anim_progress) * (1.0 - anim_progress); // ease-out quad
        let scale = COUNTDOWN_START_SCALE + (1.0 - COUNTDOWN_START_SCALE) * eased;
        transform.scale = Vec3::splat(scale);

        // Fade alpha slightly during the hold phase (after animation)
        if display.count == 0 {
            // GO! fades out
            let fade = display.timer.fraction_remaining();
            let green = Color::srgba(0.40, 1.0, 0.50, fade);
            text_color.0 = green;
        }

        if display.timer.just_finished() {
            if display.count == 0 {
                // GO! finished → start playing
                next_state.set(GameState::Playing);
            } else if display.count == 1 {
                // 1 → GO!
                display.count = 0;
                display.timer = Timer::from_seconds(COUNTDOWN_GO_DURATION, TimerMode::Once);
                **text = "GO!".to_string();
                text_color.0 = Color::srgb(0.40, 1.0, 0.50);
                transform.scale = Vec3::splat(COUNTDOWN_START_SCALE);
                play_go(&mut commands, &sounds, &audio_settings);
            } else {
                // 3→2 or 2→1
                display.count -= 1;
                display.timer = Timer::from_seconds(COUNTDOWN_STEP_DURATION, TimerMode::Once);
                **text = display.count.to_string();
                transform.scale = Vec3::splat(COUNTDOWN_START_SCALE);
                play_beep(&mut commands, &sounds, &audio_settings);
            }
        }
    }
}

/// Remove countdown display entities
pub fn cleanup_countdown(mut commands: Commands, query: Query<Entity, With<CountdownDisplay>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn play_beep(commands: &mut Commands, sounds: &GameSounds, audio_settings: &AudioSettings) {
    if let Some(source) = sounds.countdown_beep.clone() {
        commands.spawn((
            AudioPlayer::new(source),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(audio_settings.sfx_volume)),
        ));
    }
}

fn play_go(commands: &mut Commands, sounds: &GameSounds, audio_settings: &AudioSettings) {
    if let Some(source) = sounds.countdown_go.clone() {
        commands.spawn((
            AudioPlayer::new(source),
            PlaybackSettings::DESPAWN.with_volume(Volume::new(audio_settings.sfx_volume)),
        ));
    }
}
