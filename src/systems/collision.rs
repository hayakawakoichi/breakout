use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::states::GameState;
use crate::systems::audio::CollisionEvent;
use crate::systems::setup::durable_color;

/// Check AABB collision between two rectangles
fn aabb_collision(pos_a: Vec2, size_a: Vec2, pos_b: Vec2, size_b: Vec2) -> bool {
    let half_a = size_a / 2.0;
    let half_b = size_b / 2.0;

    pos_a.x - half_a.x < pos_b.x + half_b.x
        && pos_a.x + half_a.x > pos_b.x - half_b.x
        && pos_a.y - half_a.y < pos_b.y + half_b.y
        && pos_a.y + half_a.y > pos_b.y - half_b.y
}

/// Handle ball-paddle collision (multi-ball support)
pub fn ball_paddle_collision(
    mut ball_query: Query<(&Transform, &mut Velocity, &Collider), With<Ball>>,
    paddle_query: Query<(&Transform, &Collider), With<Paddle>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let Ok((paddle_transform, paddle_collider)) = paddle_query.get_single() else {
        return;
    };

    for (ball_transform, mut ball_velocity, ball_collider) in &mut ball_query {
        if aabb_collision(
            ball_transform.translation.truncate(),
            ball_collider.size,
            paddle_transform.translation.truncate(),
            paddle_collider.size,
        ) {
            // Only bounce if ball is moving downward
            if ball_velocity.0.y < 0.0 {
                // Reflect Y direction
                ball_velocity.0.y = ball_velocity.0.y.abs();

                // Adjust X based on hit position on paddle
                let hit_pos = ball_transform.translation.x - paddle_transform.translation.x;
                let paddle_width = paddle_collider.size.x;
                let normalized = hit_pos / (paddle_width / 2.0);
                let speed = ball_velocity.0.length();
                ball_velocity.0.x = normalized * speed * 0.8;

                // Normalize to maintain current speed (preserves slow ball effect)
                ball_velocity.0 = ball_velocity.0.normalize() * speed;

                collision_events.send(CollisionEvent::Paddle);
            }
        }
    }
}

/// Handle ball-wall collision (multi-ball support)
/// Uses position check for bottom to prevent tunneling at high speeds
pub fn ball_wall_collision(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &Transform, &mut Velocity, &Collider), With<Ball>>,
    wall_query: Query<(&Transform, &Collider, &Wall)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let total_balls = ball_query.iter().count();
    let mut balls_lost = 0;
    let bottom_limit = -WINDOW_HEIGHT / 2.0;

    for (ball_entity, ball_transform, mut ball_velocity, ball_collider) in &mut ball_query {
        // Position-based bottom check (no tunneling possible)
        if ball_transform.translation.y < bottom_limit {
            commands.entity(ball_entity).despawn();
            balls_lost += 1;
            continue;
        }

        for (wall_transform, wall_collider, wall_type) in &wall_query {
            if aabb_collision(
                ball_transform.translation.truncate(),
                ball_collider.size,
                wall_transform.translation.truncate(),
                wall_collider.size,
            ) {
                match wall_type {
                    Wall::Top => {
                        ball_velocity.0.y = -ball_velocity.0.y.abs();
                        collision_events.send(CollisionEvent::Wall);
                    }
                    Wall::Left => {
                        ball_velocity.0.x = ball_velocity.0.x.abs();
                        collision_events.send(CollisionEvent::Wall);
                    }
                    Wall::Right => {
                        ball_velocity.0.x = -ball_velocity.0.x.abs();
                        collision_events.send(CollisionEvent::Wall);
                    }
                    Wall::Bottom => {
                        // Handled by position check above
                    }
                }
            }
        }
    }

    if balls_lost > 0 && balls_lost >= total_balls {
        collision_events.send(CollisionEvent::GameOver);
        next_state.set(GameState::GameOver);
    }
}

/// Handle ball-block collision (multi-ball support + power-up drops + special block types)
pub fn ball_block_collision(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &Transform, &mut Velocity, &Collider), With<Ball>>,
    mut block_query: Query<(Entity, &Transform, &Collider, &mut Sprite, &mut Block)>,
    mut score: ResMut<Score>,
    mut combo: ResMut<ComboTracker>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut screen_shake: ResMut<crate::resources::ScreenShake>,
) {
    // Track which blocks have been destroyed this frame to avoid double-processing
    let mut destroyed_blocks = Vec::new();

    for (_, ball_transform, mut ball_velocity, ball_collider) in &mut ball_query {
        let mut hit_block = false;

        for (block_entity, block_transform, block_collider, mut block_sprite, mut block) in
            &mut block_query
        {
            if destroyed_blocks.contains(&block_entity) {
                continue;
            }

            if aabb_collision(
                ball_transform.translation.truncate(),
                ball_collider.size,
                block_transform.translation.truncate(),
                block_collider.size,
            ) {
                let block_pos = block_transform.translation.truncate();
                let ball_pos = ball_transform.translation.truncate();
                let diff = ball_pos - block_pos;
                let x_overlap = (BLOCK_WIDTH + BALL_SIZE) / 2.0 - diff.x.abs();
                let y_overlap = (BLOCK_HEIGHT + BALL_SIZE) / 2.0 - diff.y.abs();

                match block.block_type {
                    BlockType::Steel => {
                        // Wall-like robust reflection: set absolute direction to push ball out
                        if x_overlap < y_overlap {
                            if diff.x > 0.0 {
                                ball_velocity.0.x = ball_velocity.0.x.abs();
                            } else {
                                ball_velocity.0.x = -ball_velocity.0.x.abs();
                            }
                        } else {
                            if diff.y > 0.0 {
                                ball_velocity.0.y = ball_velocity.0.y.abs();
                            } else {
                                ball_velocity.0.y = -ball_velocity.0.y.abs();
                            }
                        }
                        collision_events.send(CollisionEvent::Wall);
                    }
                    _ => {
                        // Standard reflection for breakable blocks
                        if x_overlap < y_overlap {
                            ball_velocity.0.x = -ball_velocity.0.x;
                        } else {
                            ball_velocity.0.y = -ball_velocity.0.y;
                        }
                    }
                }

                match block.block_type {
                    BlockType::Normal => {
                        let block_color = block_sprite.color;
                        commands.entity(block_entity).despawn();
                        destroyed_blocks.push(block_entity);
                        spawn_particles(&mut commands, block_pos, block_color);
                        screen_shake.trauma = (screen_shake.trauma + SHAKE_TRAUMA).min(1.0);

                        // Combo scoring
                        combo.count += 1;
                        combo.timer.reset();
                        let multiplier = combo.count;
                        let gained = SCORE_PER_BLOCK * multiplier;
                        score.value += gained;
                        combo.last_score_gained = gained;

                        collision_events.send(CollisionEvent::Block);

                        if rand_f32() < POWERUP_DROP_CHANCE {
                            spawn_powerup(&mut commands, block_pos);
                        }
                    }
                    BlockType::Durable { hits_remaining } => {
                        if hits_remaining <= 1 {
                            // Destroy the block
                            let block_color = block_sprite.color;
                            commands.entity(block_entity).despawn();
                            destroyed_blocks.push(block_entity);
                            spawn_particles(&mut commands, block_pos, block_color);
                            screen_shake.trauma =
                                (screen_shake.trauma + SHAKE_TRAUMA).min(1.0);

                            combo.count += 1;
                            combo.timer.reset();
                            let multiplier = combo.count;
                            let gained = SCORE_PER_BLOCK * multiplier + DURABLE_SCORE_BONUS;
                            score.value += gained;
                            combo.last_score_gained = gained;

                            collision_events.send(CollisionEvent::Block);

                            if rand_f32() < POWERUP_DROP_CHANCE {
                                spawn_powerup(&mut commands, block_pos);
                            }
                        } else {
                            // Reduce hits and change color
                            block.block_type = BlockType::Durable {
                                hits_remaining: hits_remaining - 1,
                            };
                            block_sprite.color = durable_color(hits_remaining - 1);
                            collision_events.send(CollisionEvent::Wall); // lighter hit sound
                        }
                    }
                    BlockType::Steel => {
                        // Already handled above (reflection only)
                    }
                    BlockType::Explosive => {
                        let block_color = block_sprite.color;
                        commands.entity(block_entity).despawn();
                        destroyed_blocks.push(block_entity);
                        spawn_particles(&mut commands, block_pos, block_color);
                        screen_shake.trauma = (screen_shake.trauma + SHAKE_TRAUMA * 1.5).min(1.0);

                        combo.count += 1;
                        combo.timer.reset();
                        let multiplier = combo.count;
                        let gained = SCORE_PER_BLOCK * multiplier;
                        score.value += gained;
                        combo.last_score_gained = gained;

                        collision_events.send(CollisionEvent::Block);

                        // Chain explosion — collect blocks in radius
                        let mut explosion_queue = vec![block_pos];
                        process_explosions(
                            &mut commands,
                            &mut block_query,
                            &mut destroyed_blocks,
                            &mut explosion_queue,
                            &mut score,
                            &mut combo,
                            &mut collision_events,
                            &mut screen_shake,
                        );
                    }
                }

                hit_block = true;
                // Only process one collision per ball per frame
                break;
            }
        }

        if hit_block {
            continue;
        }
    }
}

/// Process explosion chain reactions
fn process_explosions(
    commands: &mut Commands,
    block_query: &mut Query<(Entity, &Transform, &Collider, &mut Sprite, &mut Block)>,
    destroyed_blocks: &mut Vec<Entity>,
    explosion_queue: &mut Vec<Vec2>,
    score: &mut ResMut<Score>,
    combo: &mut ResMut<ComboTracker>,
    collision_events: &mut EventWriter<CollisionEvent>,
    screen_shake: &mut ResMut<ScreenShake>,
) {
    let mut queue_idx = 0;
    while queue_idx < explosion_queue.len() {
        let explosion_pos = explosion_queue[queue_idx];
        queue_idx += 1;

        // Find blocks within explosion radius
        let mut to_destroy = Vec::new();
        for (entity, transform, _, _, block) in block_query.iter() {
            if destroyed_blocks.contains(&entity) {
                continue;
            }
            let dist = transform.translation.truncate().distance(explosion_pos);
            if dist <= EXPLOSIVE_RADIUS {
                match block.block_type {
                    BlockType::Steel => {} // Steel is immune to explosions
                    _ => {
                        to_destroy.push((entity, transform.translation.truncate()));
                    }
                }
            }
        }

        // Destroy collected blocks
        for (entity, pos) in to_destroy {
            if destroyed_blocks.contains(&entity) {
                continue;
            }

            // Read block info before despawning
            let (block_type, block_color) = {
                if let Ok((_, _, _, sprite, block)) = block_query.get(entity) {
                    (block.block_type, sprite.color)
                } else {
                    continue;
                }
            };

            commands.entity(entity).despawn();
            destroyed_blocks.push(entity);
            spawn_particles(commands, pos, block_color);

            combo.count += 1;
            combo.timer.reset();
            let multiplier = combo.count;
            let gained = SCORE_PER_BLOCK * multiplier;
            score.value += gained;
            combo.last_score_gained = gained;

            collision_events.send(CollisionEvent::Block);
            screen_shake.trauma = (screen_shake.trauma + SHAKE_TRAUMA * 0.5).min(1.0);

            // If the destroyed block is also Explosive, add to chain
            if matches!(block_type, BlockType::Explosive) {
                explosion_queue.push(pos);
            }
        }
    }
}

/// Spawn a random power-up at the given position
fn spawn_powerup(commands: &mut Commands, position: Vec2) {
    let roll = rand_f32();
    let power_type = if roll < 0.33 {
        PowerUpType::WidePaddle
    } else if roll < 0.66 {
        PowerUpType::MultiBall
    } else {
        PowerUpType::SlowBall
    };

    let color = match power_type {
        PowerUpType::WidePaddle => Color::srgb(0.95, 0.40, 0.80),  // Magenta
        PowerUpType::MultiBall => Color::srgb(0.40, 0.90, 0.95),   // Cyan
        PowerUpType::SlowBall => Color::srgb(0.60, 0.95, 0.40),    // Lime
    };

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::splat(POWERUP_SIZE)),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, 0.5),
        PowerUp { power_type },
        Velocity(Vec2::new(0.0, -POWERUP_FALL_SPEED)),
        Collider {
            size: Vec2::splat(POWERUP_SIZE),
        },
    ));
}

/// Spawn particle effects at the given position with the given color
fn spawn_particles(commands: &mut Commands, position: Vec2, color: Color) {
    use std::f32::consts::TAU;
    use crate::components::Particle;

    for i in 0..PARTICLE_COUNT {
        // Spread particles evenly around a circle with some randomness
        let base_angle = (i as f32 / PARTICLE_COUNT as f32) * TAU;
        let angle = base_angle + simple_rand(i as u32) * 0.5;
        let speed = PARTICLE_SPEED * (0.5 + simple_rand(i as u32 + 100) * 0.5);
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 1.0),
            Particle {
                lifetime: Timer::from_seconds(PARTICLE_LIFETIME, TimerMode::Once),
                velocity,
            },
        ));
    }
}

/// Simple deterministic random-ish value in [0, 1) from a seed
fn simple_rand(seed: u32) -> f32 {
    let n = seed.wrapping_mul(1103515245).wrapping_add(12345);
    (n & 0x7FFFFFFF) as f32 / 0x7FFFFFFF as f32
}

/// WASM-compatible pseudo-random f32 in [0, 1) using an atomic counter
fn rand_f32() -> f32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(54321);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let hash = n.wrapping_mul(1103515245).wrapping_add(12345);
    (hash & 0x7FFFFFFF) as f32 / 0x7FFFFFFF as f32
}

/// Check if all non-Steel blocks are destroyed
pub fn check_level_clear(
    block_query: Query<&Block>,
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let remaining = block_query
        .iter()
        .filter(|b| !matches!(b.block_type, BlockType::Steel))
        .count();
    if remaining == 0 {
        collision_events.send(CollisionEvent::LevelClear);
        next_state.set(GameState::LevelClear);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    // --- aabb_collision unit tests ---

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
    fn aabb_collision_touching_edge() {
        // Touching edges: half_a.x + half_b.x == distance → not overlapping (strict <)
        assert!(!aabb_collision(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(10.0, 0.0),
            Vec2::new(10.0, 10.0),
        ));
    }

    #[test]
    fn aabb_collision_contained() {
        assert!(aabb_collision(
            Vec2::new(0.0, 0.0),
            Vec2::new(20.0, 20.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 2.0),
        ));
    }

    // --- simple_rand / rand_f32 ---

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
        assert_eq!(simple_rand(0), simple_rand(0));
    }

    // --- ball_paddle_collision ---

    /// Y position that ensures ball overlaps with paddle for AABB collision
    fn ball_y_overlapping_paddle() -> f32 {
        // Place ball just barely overlapping: paddle_center + half_paddle + half_ball - small_gap
        PADDLE_Y + (PADDLE_HEIGHT + BALL_SIZE) / 2.0 - 2.0
    }

    #[test]
    fn ball_bounces_off_paddle_center() {
        let mut app = test_app();
        spawn_test_paddle(app.world_mut(), 0.0);
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, ball_y_overlapping_paddle()),
            Vec2::new(0.0, -BALL_SPEED),
        );

        app.add_systems(Update, ball_paddle_collision);
        app.update();

        let ball_vel = app
            .world_mut()
            .query::<&Velocity>()
            .iter(app.world())
            .next()
            .unwrap();
        assert!(ball_vel.0.y > 0.0, "Ball should bounce upward");
    }

    #[test]
    fn ball_no_bounce_when_moving_up() {
        let mut app = test_app();
        spawn_test_paddle(app.world_mut(), 0.0);
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, ball_y_overlapping_paddle()),
            Vec2::new(0.0, BALL_SPEED), // Moving upward
        );

        app.add_systems(Update, ball_paddle_collision);
        app.update();

        let ball_vel = app
            .world_mut()
            .query::<&Velocity>()
            .iter(app.world())
            .next()
            .unwrap();
        assert!(
            ball_vel.0.y > 0.0,
            "Ball moving up should not be affected"
        );
    }

    #[test]
    fn ball_angle_from_paddle_hit_pos() {
        let mut app = test_app();
        spawn_test_paddle(app.world_mut(), 0.0);
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(30.0, ball_y_overlapping_paddle()),
            Vec2::new(0.0, -BALL_SPEED),
        );

        app.add_systems(Update, ball_paddle_collision);
        app.update();

        let ball_vel = app
            .world_mut()
            .query::<&Velocity>()
            .iter(app.world())
            .next()
            .unwrap();
        assert!(ball_vel.0.x > 0.0, "Hit right side → positive x velocity");
    }

    #[test]
    fn ball_speed_preserved_after_paddle() {
        let mut app = test_app();
        spawn_test_paddle(app.world_mut(), 0.0);
        let initial_speed = BALL_SPEED;
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(10.0, ball_y_overlapping_paddle()),
            Vec2::new(0.0, -initial_speed),
        );

        app.add_systems(Update, ball_paddle_collision);
        app.update();

        let ball_vel = app
            .world_mut()
            .query::<&Velocity>()
            .iter(app.world())
            .next()
            .unwrap();
        let speed_after = ball_vel.0.length();
        assert!(
            (speed_after - initial_speed).abs() < 1.0,
            "Speed should be preserved: {speed_after} vs {initial_speed}"
        );
    }

    // --- ball_wall_collision ---

    #[test]
    fn ball_bounces_off_top_wall() {
        let mut app = test_app();
        let wall_y = WINDOW_HEIGHT / 2.0;
        spawn_test_wall(
            app.world_mut(),
            Wall::Top,
            Vec2::new(0.0, wall_y),
            Vec2::new(WINDOW_WIDTH, WALL_THICKNESS),
        );
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, wall_y - WALL_THICKNESS),
            Vec2::new(100.0, BALL_SPEED),
        );

        app.add_systems(
            Update,
            ball_wall_collision.before(collect_collision_events),
        );
        app.add_systems(Update, collect_collision_events);
        app.init_resource::<CollectedEvents>();
        app.update();

        let ball_vel = app
            .world_mut()
            .query::<&Velocity>()
            .iter(app.world())
            .next()
            .unwrap();
        assert!(ball_vel.0.y < 0.0, "Ball should bounce downward off top wall");
    }

    #[test]
    fn ball_bounces_off_left_wall() {
        let mut app = test_app();
        let wall_x = -WINDOW_WIDTH / 2.0;
        spawn_test_wall(
            app.world_mut(),
            Wall::Left,
            Vec2::new(wall_x, 0.0),
            Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT),
        );
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(wall_x + WALL_THICKNESS, 0.0),
            Vec2::new(-BALL_SPEED, 100.0),
        );

        app.add_systems(
            Update,
            ball_wall_collision.before(collect_collision_events),
        );
        app.add_systems(Update, collect_collision_events);
        app.init_resource::<CollectedEvents>();
        app.update();

        let ball_vel = app
            .world_mut()
            .query::<&Velocity>()
            .iter(app.world())
            .next()
            .unwrap();
        assert!(ball_vel.0.x > 0.0, "Ball should bounce right off left wall");
    }

    #[test]
    fn ball_bounces_off_right_wall() {
        let mut app = test_app();
        let wall_x = WINDOW_WIDTH / 2.0;
        spawn_test_wall(
            app.world_mut(),
            Wall::Right,
            Vec2::new(wall_x, 0.0),
            Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT),
        );
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(wall_x - WALL_THICKNESS, 0.0),
            Vec2::new(BALL_SPEED, 100.0),
        );

        app.add_systems(
            Update,
            ball_wall_collision.before(collect_collision_events),
        );
        app.add_systems(Update, collect_collision_events);
        app.init_resource::<CollectedEvents>();
        app.update();

        let ball_vel = app
            .world_mut()
            .query::<&Velocity>()
            .iter(app.world())
            .next()
            .unwrap();
        assert!(ball_vel.0.x < 0.0, "Ball should bounce left off right wall");
    }

    #[test]
    fn ball_despawn_on_bottom_game_over() {
        let mut app = test_app();
        // Place ball below the bottom limit
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, -WINDOW_HEIGHT / 2.0 - 10.0),
            Vec2::new(0.0, -BALL_SPEED),
        );

        app.add_systems(
            Update,
            ball_wall_collision.before(collect_collision_events),
        );
        app.add_systems(Update, collect_collision_events);
        app.init_resource::<CollectedEvents>();
        app.update();

        let ball_count = app
            .world_mut()
            .query::<&Ball>()
            .iter(app.world())
            .count();
        assert_eq!(ball_count, 0, "Ball should be despawned");

        let events = app.world().resource::<CollectedEvents>();
        assert!(events.events.contains(&CollisionEvent::GameOver));
    }

    #[test]
    fn multi_ball_one_lost_no_game_over() {
        let mut app = test_app();
        // Ball 1: below bottom (will be lost)
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, -WINDOW_HEIGHT / 2.0 - 10.0),
            Vec2::new(0.0, -BALL_SPEED),
        );
        // Ball 2: safe position
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 0.0),
            Vec2::new(BALL_SPEED, BALL_SPEED),
        );

        app.add_systems(
            Update,
            ball_wall_collision.before(collect_collision_events),
        );
        app.add_systems(Update, collect_collision_events);
        app.init_resource::<CollectedEvents>();
        app.update();

        let ball_count = app
            .world_mut()
            .query::<&Ball>()
            .iter(app.world())
            .count();
        assert_eq!(ball_count, 1, "One ball should remain");

        let events = app.world().resource::<CollectedEvents>();
        assert!(
            !events.events.contains(&CollisionEvent::GameOver),
            "Should not game over with remaining balls"
        );
    }

    // --- ball_block_collision ---

    #[test]
    fn block_hit_adds_score_with_combo() {
        let mut app = test_app();
        spawn_test_block(app.world_mut(), Vec2::new(0.0, 100.0));
        // Ball overlapping the block, moving upward
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 100.0),
            Vec2::new(0.0, BALL_SPEED),
        );

        app.add_systems(Update, (ball_block_collision, collect_collision_events));
        app.init_resource::<CollectedEvents>();
        app.update();

        let score = app.world().resource::<Score>();
        // First hit: combo count becomes 1, score = SCORE_PER_BLOCK * 1 = 10
        assert_eq!(score.value, SCORE_PER_BLOCK);
    }

    #[test]
    fn block_hit_reverses_velocity() {
        let mut app = test_app();
        // Ball approaching block from below (y_overlap < x_overlap → reverse y)
        spawn_test_block(app.world_mut(), Vec2::new(0.0, 100.0));
        let initial_vy = BALL_SPEED;
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 100.0 - BLOCK_HEIGHT / 2.0 - BALL_SIZE / 2.0 + 2.0),
            Vec2::new(0.0, initial_vy),
        );

        app.add_systems(Update, ball_block_collision);
        app.update();

        let ball_vel = app
            .world_mut()
            .query::<&Velocity>()
            .iter(app.world())
            .next()
            .unwrap();
        assert!(
            ball_vel.0.y < 0.0,
            "y velocity should reverse after block hit"
        );
    }

    #[test]
    fn block_hit_triggers_screen_shake() {
        let mut app = test_app();
        spawn_test_block(app.world_mut(), Vec2::new(0.0, 100.0));
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 100.0),
            Vec2::new(0.0, BALL_SPEED),
        );

        app.add_systems(Update, ball_block_collision);
        app.update();

        let shake = app.world().resource::<ScreenShake>();
        assert!(shake.trauma > 0.0, "Screen shake should be triggered");
    }

    #[test]
    fn one_block_per_ball_per_frame() {
        let mut app = test_app();
        // Two blocks close together
        spawn_test_block(app.world_mut(), Vec2::new(0.0, 100.0));
        spawn_test_block(app.world_mut(), Vec2::new(0.0, 100.0 + BLOCK_HEIGHT + 1.0));
        // Ball overlapping both
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 100.0 + BLOCK_HEIGHT / 2.0),
            Vec2::new(0.0, BALL_SPEED),
        );

        app.add_systems(Update, ball_block_collision);
        app.update();

        let block_count = app
            .world_mut()
            .query::<&Block>()
            .iter(app.world())
            .count();
        // At most 1 block should be destroyed per ball per frame
        assert!(
            block_count >= 1,
            "At most one block destroyed per ball per frame"
        );
    }

    // --- Special block types ---

    #[test]
    fn steel_block_reflects_but_survives() {
        let mut app = test_app();
        spawn_test_block_typed(app.world_mut(), Vec2::new(0.0, 100.0), BlockType::Steel);
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 100.0 - BLOCK_HEIGHT / 2.0 - BALL_SIZE / 2.0 + 2.0),
            Vec2::new(0.0, BALL_SPEED),
        );

        app.add_systems(
            Update,
            ball_block_collision.before(collect_collision_events),
        );
        app.add_systems(Update, collect_collision_events);
        app.init_resource::<CollectedEvents>();
        app.update();

        // Steel block should still exist
        let block_count = app
            .world_mut()
            .query::<&Block>()
            .iter(app.world())
            .count();
        assert_eq!(block_count, 1, "Steel block should survive");

        // Score should not increase
        let score = app.world().resource::<Score>();
        assert_eq!(score.value, 0, "Steel blocks give no score");

        // Should emit Wall event (not Block)
        let events = app.world().resource::<CollectedEvents>();
        assert!(events.events.contains(&CollisionEvent::Wall));
        assert!(!events.events.contains(&CollisionEvent::Block));
    }

    #[test]
    fn durable_block_loses_hit() {
        let mut app = test_app();
        spawn_test_block_typed(
            app.world_mut(),
            Vec2::new(0.0, 100.0),
            BlockType::Durable { hits_remaining: 3 },
        );
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 100.0 - BLOCK_HEIGHT / 2.0 - BALL_SIZE / 2.0 + 2.0),
            Vec2::new(0.0, BALL_SPEED),
        );

        app.add_systems(Update, ball_block_collision);
        app.update();

        // Block should still exist with reduced hits
        let block = app
            .world_mut()
            .query::<&Block>()
            .iter(app.world())
            .next()
            .unwrap();
        assert_eq!(
            block.block_type,
            BlockType::Durable { hits_remaining: 2 },
            "Hits should decrease by 1"
        );

        // Score should not increase (not destroyed yet)
        let score = app.world().resource::<Score>();
        assert_eq!(score.value, 0);
    }

    #[test]
    fn durable_block_destroyed_at_one_hit() {
        let mut app = test_app();
        spawn_test_block_typed(
            app.world_mut(),
            Vec2::new(0.0, 100.0),
            BlockType::Durable { hits_remaining: 1 },
        );
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 100.0),
            Vec2::new(0.0, BALL_SPEED),
        );

        app.add_systems(Update, ball_block_collision);
        app.update();

        let block_count = app
            .world_mut()
            .query::<&Block>()
            .iter(app.world())
            .count();
        assert_eq!(block_count, 0, "Durable block at 1 hit should be destroyed");

        // Should get score with durable bonus
        let score = app.world().resource::<Score>();
        assert_eq!(
            score.value,
            SCORE_PER_BLOCK + DURABLE_SCORE_BONUS,
            "Durable destruction gives bonus"
        );
    }

    #[test]
    fn explosive_block_chain_destroys_nearby() {
        let mut app = test_app();
        // Explosive block at origin
        spawn_test_block_typed(app.world_mut(), Vec2::new(0.0, 100.0), BlockType::Explosive);
        // Normal block within explosion radius
        spawn_test_block(app.world_mut(), Vec2::new(50.0, 100.0));
        // Normal block outside explosion radius
        spawn_test_block(app.world_mut(), Vec2::new(300.0, 100.0));

        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 100.0),
            Vec2::new(0.0, BALL_SPEED),
        );

        app.add_systems(Update, ball_block_collision);
        app.update();

        let block_count = app
            .world_mut()
            .query::<&Block>()
            .iter(app.world())
            .count();
        // Explosive + nearby normal destroyed, far normal survives
        assert_eq!(
            block_count, 1,
            "Only the far block should survive the explosion"
        );
    }

    #[test]
    fn combo_scoring_increments() {
        let mut app = test_app();
        // Two blocks at different positions
        spawn_test_block(app.world_mut(), Vec2::new(0.0, 100.0));
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 100.0),
            Vec2::new(0.0, BALL_SPEED),
        );

        app.add_systems(Update, ball_block_collision);
        app.update();

        let combo = app.world().resource::<ComboTracker>();
        assert_eq!(combo.count, 1, "Combo count should be 1 after first hit");
        assert_eq!(combo.last_score_gained, SCORE_PER_BLOCK);
    }

    // --- check_level_clear ---

    #[test]
    fn level_clear_when_no_blocks() {
        let mut app = test_app();
        // No blocks at all
        app.add_systems(
            Update,
            check_level_clear.before(collect_collision_events),
        );
        app.add_systems(Update, collect_collision_events);
        app.init_resource::<CollectedEvents>();
        app.update();

        let events = app.world().resource::<CollectedEvents>();
        assert!(events.events.contains(&CollisionEvent::LevelClear));
    }

    #[test]
    fn no_level_clear_with_blocks() {
        let mut app = test_app();
        spawn_test_block(app.world_mut(), Vec2::new(0.0, 100.0));

        app.add_systems(
            Update,
            check_level_clear.before(collect_collision_events),
        );
        app.add_systems(Update, collect_collision_events);
        app.init_resource::<CollectedEvents>();
        app.update();

        let events = app.world().resource::<CollectedEvents>();
        assert!(
            !events.events.contains(&CollisionEvent::LevelClear),
            "Should not clear level with remaining blocks"
        );
    }

    #[test]
    fn level_clear_ignores_steel_blocks() {
        let mut app = test_app();
        // Only steel blocks remain
        spawn_test_block_typed(app.world_mut(), Vec2::new(0.0, 100.0), BlockType::Steel);
        spawn_test_block_typed(app.world_mut(), Vec2::new(100.0, 100.0), BlockType::Steel);

        app.add_systems(
            Update,
            check_level_clear.before(collect_collision_events),
        );
        app.add_systems(Update, collect_collision_events);
        app.init_resource::<CollectedEvents>();
        app.update();

        let events = app.world().resource::<CollectedEvents>();
        assert!(
            events.events.contains(&CollisionEvent::LevelClear),
            "Steel-only should trigger level clear"
        );
    }
}
