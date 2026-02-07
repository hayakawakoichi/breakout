use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::states::GameState;
use crate::systems::audio::CollisionEvent;

/// Check AABB collision between two rectangles
fn aabb_collision(pos_a: Vec2, size_a: Vec2, pos_b: Vec2, size_b: Vec2) -> bool {
    let half_a = size_a / 2.0;
    let half_b = size_b / 2.0;

    pos_a.x - half_a.x < pos_b.x + half_b.x
        && pos_a.x + half_a.x > pos_b.x - half_b.x
        && pos_a.y - half_a.y < pos_b.y + half_b.y
        && pos_a.y + half_a.y > pos_b.y - half_b.y
}

/// Handle ball-paddle collision
pub fn ball_paddle_collision(
    mut ball_query: Query<(&Transform, &mut Velocity, &Collider), With<Ball>>,
    paddle_query: Query<(&Transform, &Collider), With<Paddle>>,
    level: Res<Level>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let Ok((ball_transform, mut ball_velocity, ball_collider)) = ball_query.get_single_mut() else {
        return;
    };
    let Ok((paddle_transform, paddle_collider)) = paddle_query.get_single() else {
        return;
    };

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

            // Adjust X based on hit position
            let hit_pos = ball_transform.translation.x - paddle_transform.translation.x;
            let normalized = hit_pos / (PADDLE_WIDTH / 2.0);
            let speed = BALL_SPEED * level.speed_multiplier();
            ball_velocity.0.x = normalized * speed * 0.8;

            // Normalize to maintain constant speed
            ball_velocity.0 = ball_velocity.0.normalize() * speed;

            collision_events.send(CollisionEvent::Paddle);
        }
    }
}

/// Handle ball-wall collision
pub fn ball_wall_collision(
    mut ball_query: Query<(&Transform, &mut Velocity, &Collider), With<Ball>>,
    wall_query: Query<(&Transform, &Collider, &Wall)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let Ok((ball_transform, mut ball_velocity, ball_collider)) = ball_query.get_single_mut() else {
        return;
    };

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
                    collision_events.send(CollisionEvent::GameOver);
                    next_state.set(GameState::GameOver);
                }
            }
        }
    }
}

/// Handle ball-block collision
pub fn ball_block_collision(
    mut commands: Commands,
    mut ball_query: Query<(&Transform, &mut Velocity, &Collider), With<Ball>>,
    block_query: Query<(Entity, &Transform, &Collider), With<Block>>,
    mut score: ResMut<Score>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let Ok((ball_transform, mut ball_velocity, ball_collider)) = ball_query.get_single_mut() else {
        return;
    };

    for (block_entity, block_transform, block_collider) in &block_query {
        if aabb_collision(
            ball_transform.translation.truncate(),
            ball_collider.size,
            block_transform.translation.truncate(),
            block_collider.size,
        ) {
            // Despawn the block
            commands.entity(block_entity).despawn();

            // Add score
            score.value += SCORE_PER_BLOCK;

            // Determine reflection direction
            let ball_pos = ball_transform.translation.truncate();
            let block_pos = block_transform.translation.truncate();
            let diff = ball_pos - block_pos;

            // Check which side was hit
            let x_overlap = (BLOCK_WIDTH + BALL_SIZE) / 2.0 - diff.x.abs();
            let y_overlap = (BLOCK_HEIGHT + BALL_SIZE) / 2.0 - diff.y.abs();

            if x_overlap < y_overlap {
                ball_velocity.0.x = -ball_velocity.0.x;
            } else {
                ball_velocity.0.y = -ball_velocity.0.y;
            }

            collision_events.send(CollisionEvent::Block);

            // Only process one collision per frame
            break;
        }
    }
}

/// Check if all blocks are destroyed
pub fn check_level_clear(
    block_query: Query<&Block>,
    mut next_state: ResMut<NextState<GameState>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    if block_query.is_empty() {
        collision_events.send(CollisionEvent::LevelClear);
        next_state.set(GameState::LevelClear);
    }
}
