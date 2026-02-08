use bevy::prelude::*;

/// Check AABB collision between two rectangles
pub fn aabb_collision(pos_a: Vec2, size_a: Vec2, pos_b: Vec2, size_b: Vec2) -> bool {
    let half_a = size_a / 2.0;
    let half_b = size_b / 2.0;

    pos_a.x - half_a.x < pos_b.x + half_b.x
        && pos_a.x + half_a.x > pos_b.x - half_b.x
        && pos_a.y - half_a.y < pos_b.y + half_b.y
        && pos_a.y + half_a.y > pos_b.y - half_b.y
}

/// Simple deterministic random-ish value in [0, 1) from a seed
pub fn simple_rand(seed: u32) -> f32 {
    let n = seed.wrapping_mul(1103515245).wrapping_add(12345);
    (n & 0x7FFFFFFF) as f32 / 0x7FFFFFFF as f32
}

/// WASM-compatible pseudo-random f32 in [0, 1) using an atomic counter
pub fn rand_f32() -> f32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(54321);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let hash = n.wrapping_mul(1103515245).wrapping_add(12345);
    (hash & 0x7FFFFFFF) as f32 / 0x7FFFFFFF as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabb_collision_overlapping() {
        assert!(aabb_collision(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(10.0, 10.0),
        ));
    }

    #[test]
    fn aabb_collision_non_overlapping() {
        assert!(!aabb_collision(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(20.0, 20.0),
            Vec2::new(10.0, 10.0),
        ));
    }

    #[test]
    fn simple_rand_range() {
        for seed in 0..1000 {
            let val = simple_rand(seed);
            assert!(val >= 0.0 && val < 1.0, "seed {seed} produced {val}");
        }
    }

    #[test]
    fn simple_rand_deterministic() {
        assert_eq!(simple_rand(42), simple_rand(42));
    }

    #[test]
    fn rand_f32_in_range() {
        for _ in 0..100 {
            let val = rand_f32();
            assert!(val >= 0.0 && val < 1.0);
        }
    }
}
