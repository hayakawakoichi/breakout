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
