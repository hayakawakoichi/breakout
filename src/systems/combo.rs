use bevy::prelude::*;
use bevy::text::FontSmoothing;

use crate::components::ComboPopup;
use crate::constants::*;
use crate::resources::*;

/// Tick the combo timer and reset count when it expires
pub fn update_combo_timer(
    time: Res<Time>,
    mut combo: ResMut<ComboTracker>,
) {
    if combo.count > 0 {
        combo.timer.tick(time.delta());
        if combo.timer.finished() {
            combo.count = 0;
        }
    }
}

/// Spawn a combo popup when combo count reaches 2+
pub fn spawn_combo_popup(
    mut commands: Commands,
    combo: Res<ComboTracker>,
    asset_server: Res<AssetServer>,
    existing_popups: Query<Entity, With<ComboPopup>>,
) {
    if !combo.is_changed() || combo.count < 2 {
        return;
    }

    // Remove previous popup before spawning new one
    for entity in &existing_popups {
        commands.entity(entity).despawn();
    }

    let font_handle: Handle<Font> = asset_server.load(GAME_FONT_PATH);
    let points = combo.last_score_gained;
    let combo_count = combo.count;

    commands.spawn((
        Text::new(format!("+{}  {}コンボ", points, combo_count)),
        TextFont {
            font: font_handle,
            font_size: 32.0,
            font_smoothing: FontSmoothing::None,
        },
        TextColor(Color::srgba(1.0, 0.90, 0.30, 1.0)), // Gold
        TextLayout::new_with_justify(JustifyText::Center),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-80.0)),
            ..default()
        },
        ComboPopup {
            timer: Timer::from_seconds(COMBO_POPUP_DURATION, TimerMode::Once),
        },
    ));
}

/// Update combo popups: fade out in place, despawn when done
pub fn update_combo_popup(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ComboPopup, &mut TextColor)>,
) {
    for (entity, mut popup, mut text_color) in &mut query {
        popup.timer.tick(time.delta());

        if popup.timer.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Fade out
        let alpha = popup.timer.fraction_remaining();
        text_color.0 = Color::srgba(1.0, 0.90, 0.30, alpha);
    }
}
