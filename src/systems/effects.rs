use bevy::prelude::*;

use crate::components::Particle;
use crate::constants::*;
use crate::resources::ScreenShake;

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

/// Simple pseudo-random f32 in [0, 1) using an atomic counter (WASM-compatible)
fn rand_f32() -> f32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(12345);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let hash = n.wrapping_mul(1103515245).wrapping_add(12345);
    (hash & 0x7FFFFFFF) as f32 / 0x7FFFFFFF as f32
}
