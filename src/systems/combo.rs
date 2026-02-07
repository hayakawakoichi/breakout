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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::test_app;

    #[test]
    fn combo_timer_ticks_when_active() {
        let mut app = test_app();
        {
            let mut combo = app.world_mut().resource_mut::<ComboTracker>();
            combo.count = 2;
            combo.timer.reset();
        }

        app.add_systems(Update, update_combo_timer);
        app.update();

        let combo = app.world().resource::<ComboTracker>();
        assert!(combo.timer.elapsed_secs() > 0.0, "Timer should tick");
        assert_eq!(combo.count, 2, "Count should not change while timer is active");
    }

    #[test]
    fn combo_resets_when_timer_expires() {
        let mut app = test_app();
        {
            let mut combo = app.world_mut().resource_mut::<ComboTracker>();
            combo.count = 5;
            // Set timer to almost expired
            combo.timer = Timer::from_seconds(0.01, TimerMode::Once);
        }

        app.add_systems(Update, update_combo_timer);
        // Tick past the timer
        app.update();
        app.update();

        let combo = app.world().resource::<ComboTracker>();
        assert_eq!(combo.count, 0, "Count should reset when timer expires");
    }

    #[test]
    fn combo_no_tick_when_zero() {
        let mut app = test_app();
        {
            let mut combo = app.world_mut().resource_mut::<ComboTracker>();
            combo.count = 0;
            combo.timer.reset();
        }

        app.add_systems(Update, update_combo_timer);
        app.update();

        let combo = app.world().resource::<ComboTracker>();
        assert_eq!(combo.count, 0);
        // Timer should not have ticked (elapsed stays at 0)
        assert!(
            combo.timer.elapsed_secs() < f32::EPSILON,
            "Timer should not tick when count is 0"
        );
    }

    #[test]
    fn combo_popup_despawns_after_duration() {
        let mut app = test_app();
        app.world_mut().spawn((
            Text::new("+10 2コンボ"),
            TextColor(Color::srgba(1.0, 0.9, 0.3, 1.0)),
            ComboPopup {
                timer: Timer::from_seconds(0.01, TimerMode::Once),
            },
        ));

        app.add_systems(Update, update_combo_popup);
        app.update();
        app.update();

        let count = app
            .world_mut()
            .query::<&ComboPopup>()
            .iter(app.world())
            .count();
        assert_eq!(count, 0, "Popup should despawn after timer expires");
    }
}
