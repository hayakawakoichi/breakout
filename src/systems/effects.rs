use bevy::prelude::*;

use crate::components::{Particle, NewRecordFlash, RankMarker};
use crate::constants::*;
use crate::resources::ScreenShake;
use crate::utils::rand_f32;

/// Update particles: move, fade out, despawn when lifetime expires
pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut particle, mut transform, mut sprite) in &mut query {
        particle.lifetime.tick(time.delta());

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Move particle
        let dt = time.delta_secs();
        transform.translation.x += particle.velocity.x * dt;
        transform.translation.y += particle.velocity.y * dt;

        // Fade out based on remaining lifetime
        let alpha = particle.lifetime.fraction_remaining();
        sprite.color = sprite.color.with_alpha(alpha);
    }
}

/// Apply screen shake to camera using trauma-based system
pub fn apply_screen_shake(
    time: Res<Time>,
    mut shake: ResMut<ScreenShake>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let dt = time.delta_secs();

    // Decay trauma over time
    shake.trauma = (shake.trauma - SHAKE_DECAY * dt).max(0.0);

    let Ok(mut camera_transform) = camera_query.get_single_mut() else {
        return;
    };

    if shake.trauma > 0.0 {
        // Use trauma² for smooth falloff
        let shake_amount = shake.trauma * shake.trauma;
        let offset_x = shake_amount * SHAKE_MAX_OFFSET * (rand_f32() * 2.0 - 1.0);
        let offset_y = shake_amount * SHAKE_MAX_OFFSET * (rand_f32() * 2.0 - 1.0);
        camera_transform.translation.x = offset_x;
        camera_transform.translation.y = offset_y;
    } else {
        // Reset camera position when shake is done
        camera_transform.translation.x = 0.0;
        camera_transform.translation.y = 0.0;
    }
}

/// Bounce the ◀ rank marker horizontally
pub fn update_rank_marker(
    time: Res<Time>,
    mut query: Query<(&mut Node, &mut RankMarker)>,
) {
    for (mut node, mut marker) in &mut query {
        marker.0 += time.delta_secs();
        // Smooth oscillation: 0px → 8px → 0px at ~0.8 Hz
        let offset = (marker.0 * 5.0).cos().mul_add(-1.0, 1.0) * 4.0;
        node.margin.left = Val::Px(offset);
    }
}

/// Blink NEW RECORD text on/off
pub fn update_new_record_flash(
    time: Res<Time>,
    mut query: Query<(&mut NewRecordFlash, &mut TextColor)>,
) {
    for (mut flash, mut text_color) in &mut query {
        flash.timer.tick(time.delta());
        if flash.timer.just_finished() {
            // Toggle visibility by alpha
            let current_alpha = text_color.0.alpha();
            if current_alpha > 0.5 {
                text_color.0 = text_color.0.with_alpha(0.0);
            } else {
                text_color.0 = text_color.0.with_alpha(1.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn particle_despawns_after_lifetime() {
        let mut app = test_app();
        app.world_mut().spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(4.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Particle {
                lifetime: Timer::from_seconds(0.01, TimerMode::Once),
                velocity: Vec2::new(100.0, 100.0),
            },
        ));

        app.add_systems(Update, update_particles);
        // Run enough updates for the short timer to expire
        app.update();
        app.update();

        let count = app
            .world_mut()
            .query::<&Particle>()
            .iter(app.world())
            .count();
        assert_eq!(count, 0, "Particle should be despawned after lifetime");
    }

    #[test]
    fn screen_shake_trauma_decays() {
        let mut app = test_app();
        app.world_mut().resource_mut::<ScreenShake>().trauma = 1.0;
        // Need a camera for the system
        app.world_mut().spawn((Camera2d, Transform::default()));

        app.add_systems(Update, apply_screen_shake);
        app.update();

        let shake = app.world().resource::<ScreenShake>();
        assert!(shake.trauma < 1.0, "Trauma should decay over time");
        assert!(shake.trauma > 0.0, "Trauma should not fully decay in one frame");
    }

    #[test]
    fn trauma_not_below_zero() {
        let mut app = test_app();
        app.world_mut().resource_mut::<ScreenShake>().trauma = 0.001;
        app.world_mut().spawn((Camera2d, Transform::default()));

        app.add_systems(Update, apply_screen_shake);
        app.update();

        let shake = app.world().resource::<ScreenShake>();
        assert!(shake.trauma >= 0.0, "Trauma should never go below 0");
    }

    #[test]
    fn camera_resets_at_zero_trauma() {
        let mut app = test_app();
        app.world_mut().resource_mut::<ScreenShake>().trauma = 0.0;
        let camera = app.world_mut().spawn((Camera2d, Transform::from_xyz(5.0, 5.0, 0.0))).id();

        app.add_systems(Update, apply_screen_shake);
        app.update();

        let transform = app.world().entity(camera).get::<Transform>().unwrap();
        assert!(
            transform.translation.x.abs() < f32::EPSILON,
            "Camera x should reset to 0"
        );
        assert!(
            transform.translation.y.abs() < f32::EPSILON,
            "Camera y should reset to 0"
        );
    }
}
