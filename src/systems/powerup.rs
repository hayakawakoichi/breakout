use bevy::prelude::*;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;

/// Move power-up items downward and despawn when off-screen
pub fn powerup_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Velocity), With<PowerUp>>,
) {
    for (entity, mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();

        // Despawn if below screen
        if transform.translation.y < -WINDOW_HEIGHT / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// Detect collision between paddle and falling power-ups
pub fn paddle_powerup_collision(
    mut commands: Commands,
    powerup_query: Query<(Entity, &Transform, &Collider, &PowerUp)>,
    mut paddle_query: Query<(Entity, &Transform, &Collider, &mut Sprite, Option<&mut PowerUpEffects>), With<Paddle>>,
    mut ball_query: Query<(&Transform, &mut Velocity), With<Ball>>,
    level: Res<Level>,
) {
    let Ok((paddle_entity, paddle_transform, paddle_collider, mut paddle_sprite, existing_effects)) =
        paddle_query.get_single_mut()
    else {
        return;
    };

    for (powerup_entity, powerup_transform, powerup_collider, powerup) in &powerup_query {
        if aabb_collision(
            powerup_transform.translation.truncate(),
            powerup_collider.size,
            paddle_transform.translation.truncate(),
            paddle_collider.size,
        ) {
            // Despawn the power-up item
            commands.entity(powerup_entity).despawn();

            match powerup.power_type {
                PowerUpType::WidePaddle => {
                    apply_wide_paddle(
                        &mut commands,
                        paddle_entity,
                        &mut paddle_sprite,
                        &existing_effects,
                    );
                }
                PowerUpType::MultiBall => {
                    spawn_extra_balls(&mut commands, &ball_query, &level);
                }
                PowerUpType::SlowBall => {
                    apply_slow_ball(
                        &mut commands,
                        paddle_entity,
                        &mut ball_query,
                        &existing_effects,
                    );
                }
            }
        }
    }
}

/// Apply wide paddle effect
fn apply_wide_paddle(
    commands: &mut Commands,
    paddle_entity: Entity,
    paddle_sprite: &mut Sprite,
    existing_effects: &Option<Mut<PowerUpEffects>>,
) {
    let wide_width = PADDLE_WIDTH * WIDE_PADDLE_MULTIPLIER;

    if let Some(effects) = existing_effects {
        // Check if already has wide paddle — reset timer if so
        let already_active = effects.effects.iter().any(|e| e.effect_type == PowerUpType::WidePaddle);
        if already_active {
            // Timer will be reset below by removing and re-inserting
        } else {
            // Apply the width change
            paddle_sprite.custom_size = Some(Vec2::new(wide_width, PADDLE_HEIGHT));
            commands.entity(paddle_entity).insert(Collider {
                size: Vec2::new(wide_width, PADDLE_HEIGHT),
            });
        }

        // Update effects: remove old WidePaddle and add fresh one
        let mut new_effects: Vec<ActiveEffect> = effects
            .effects
            .iter()
            .filter(|e| e.effect_type != PowerUpType::WidePaddle)
            .map(|e| ActiveEffect {
                effect_type: e.effect_type,
                timer: e.timer.clone(),
            })
            .collect();
        new_effects.push(ActiveEffect {
            effect_type: PowerUpType::WidePaddle,
            timer: Timer::from_seconds(WIDE_PADDLE_DURATION, TimerMode::Once),
        });
        commands.entity(paddle_entity).insert(PowerUpEffects { effects: new_effects });
    } else {
        // No effects component yet — apply width and create it
        paddle_sprite.custom_size = Some(Vec2::new(wide_width, PADDLE_HEIGHT));
        commands.entity(paddle_entity).insert(Collider {
            size: Vec2::new(wide_width, PADDLE_HEIGHT),
        });
        commands.entity(paddle_entity).insert(PowerUpEffects {
            effects: vec![ActiveEffect {
                effect_type: PowerUpType::WidePaddle,
                timer: Timer::from_seconds(WIDE_PADDLE_DURATION, TimerMode::Once),
            }],
        });
    }
}

/// Spawn 2 extra balls from existing ball position
fn spawn_extra_balls(
    commands: &mut Commands,
    ball_query: &Query<(&Transform, &mut Velocity), With<Ball>>,
    _level: &Level,
) {
    // Get the first ball's position and velocity as reference
    let Some((ball_transform, ball_velocity)) = ball_query.iter().next() else {
        return;
    };

    let pos = ball_transform.translation.truncate();
    let speed = ball_velocity.0.length();
    let base_angle = ball_velocity.0.y.atan2(ball_velocity.0.x);

    // Spawn 2 extra balls at ±30° from the original direction
    for offset in &[0.52, -0.52] { // ~30 degrees in radians
        let angle = base_angle + offset;
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.96, 0.88), // Warm white (same as original ball)
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
            Ball,
            Velocity(velocity),
            Collider {
                size: Vec2::new(BALL_SIZE, BALL_SIZE),
            },
        ));
    }
}

/// Apply slow ball effect
fn apply_slow_ball(
    commands: &mut Commands,
    paddle_entity: Entity,
    ball_query: &mut Query<(&Transform, &mut Velocity), With<Ball>>,
    existing_effects: &Option<Mut<PowerUpEffects>>,
) {
    if let Some(effects) = existing_effects {
        let already_active = effects.effects.iter().any(|e| e.effect_type == PowerUpType::SlowBall);

        if !already_active {
            // Apply slow effect to all balls
            for (_, mut velocity) in ball_query.iter_mut() {
                velocity.0 *= SLOW_BALL_MULTIPLIER;
            }
        }

        // Update effects: remove old SlowBall and add fresh one
        let mut new_effects: Vec<ActiveEffect> = effects
            .effects
            .iter()
            .filter(|e| e.effect_type != PowerUpType::SlowBall)
            .map(|e| ActiveEffect {
                effect_type: e.effect_type,
                timer: e.timer.clone(),
            })
            .collect();
        new_effects.push(ActiveEffect {
            effect_type: PowerUpType::SlowBall,
            timer: Timer::from_seconds(SLOW_BALL_DURATION, TimerMode::Once),
        });
        commands.entity(paddle_entity).insert(PowerUpEffects { effects: new_effects });
    } else {
        // Apply slow effect and create effects component
        for (_, mut velocity) in ball_query.iter_mut() {
            velocity.0 *= SLOW_BALL_MULTIPLIER;
        }
        commands.entity(paddle_entity).insert(PowerUpEffects {
            effects: vec![ActiveEffect {
                effect_type: PowerUpType::SlowBall,
                timer: Timer::from_seconds(SLOW_BALL_DURATION, TimerMode::Once),
            }],
        });
    }
}

/// Update active power-up effects: tick timers and revert when expired
pub fn update_powerup_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut paddle_query: Query<(Entity, &mut PowerUpEffects, &mut Sprite, &mut Collider), With<Paddle>>,
    mut ball_query: Query<&mut Velocity, With<Ball>>,
    level: Res<Level>,
) {
    let Ok((paddle_entity, mut effects_comp, mut sprite, mut collider)) =
        paddle_query.get_single_mut()
    else {
        return;
    };

    let mut expired = Vec::new();

    for (i, effect) in effects_comp.effects.iter_mut().enumerate() {
        effect.timer.tick(time.delta());

        if effect.timer.just_finished() {
            expired.push((i, effect.effect_type));
        }
    }

    // Process expired effects in reverse order to keep indices valid
    for (i, effect_type) in expired.iter().rev() {
        match effect_type {
            PowerUpType::WidePaddle => {
                // Restore paddle to original size
                sprite.custom_size = Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT));
                collider.size = Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT);
            }
            PowerUpType::SlowBall => {
                // Restore ball speed: normalize each ball's velocity to the correct speed
                let target_speed = BALL_SPEED * level.speed_multiplier();
                for mut velocity in &mut ball_query {
                    let dir = velocity.0.normalize_or_zero();
                    if dir != Vec2::ZERO {
                        velocity.0 = dir * target_speed;
                    }
                }
            }
            PowerUpType::MultiBall => {
                // MultiBall has no timer-based expiry
            }
        }

        effects_comp.effects.remove(*i);
    }

    // Remove the component entirely if no effects remain
    if effects_comp.effects.is_empty() {
        commands.entity(paddle_entity).remove::<PowerUpEffects>();
    }
}

/// AABB collision check
fn aabb_collision(pos_a: Vec2, size_a: Vec2, pos_b: Vec2, size_b: Vec2) -> bool {
    let half_a = size_a / 2.0;
    let half_b = size_b / 2.0;

    pos_a.x - half_a.x < pos_b.x + half_b.x
        && pos_a.x + half_a.x > pos_b.x - half_b.x
        && pos_a.y - half_a.y < pos_b.y + half_b.y
        && pos_a.y + half_a.y > pos_b.y - half_b.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn powerup_falls_down() {
        let mut app = test_app();
        let powerup = app
            .world_mut()
            .spawn((
                Sprite {
                    custom_size: Some(Vec2::splat(POWERUP_SIZE)),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0),
                PowerUp {
                    power_type: PowerUpType::WidePaddle,
                },
                Velocity(Vec2::new(0.0, -POWERUP_FALL_SPEED)),
                Collider {
                    size: Vec2::splat(POWERUP_SIZE),
                },
            ))
            .id();

        app.add_systems(Update, powerup_movement);
        app.update();

        let transform = app.world().entity(powerup).get::<Transform>().unwrap();
        assert!(transform.translation.y < 0.0, "Power-up should fall down");
    }

    #[test]
    fn powerup_despawns_below_screen() {
        let mut app = test_app();
        app.world_mut().spawn((
            Sprite {
                custom_size: Some(Vec2::splat(POWERUP_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 - 1.0, 0.0),
            PowerUp {
                power_type: PowerUpType::MultiBall,
            },
            Velocity(Vec2::new(0.0, -POWERUP_FALL_SPEED)),
            Collider {
                size: Vec2::splat(POWERUP_SIZE),
            },
        ));

        app.add_systems(Update, powerup_movement);
        app.update();

        let count = app
            .world_mut()
            .query::<&PowerUp>()
            .iter(app.world())
            .count();
        assert_eq!(count, 0, "Power-up below screen should be despawned");
    }

    #[test]
    fn wide_paddle_widens() {
        let mut app = test_app();
        let paddle = spawn_test_paddle(app.world_mut(), 0.0);
        // Spawn power-up overlapping paddle
        app.world_mut().spawn((
            Sprite {
                custom_size: Some(Vec2::splat(POWERUP_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, PADDLE_Y, 0.0),
            PowerUp {
                power_type: PowerUpType::WidePaddle,
            },
            Collider {
                size: Vec2::splat(POWERUP_SIZE),
            },
        ));
        // Need a ball for the system query
        spawn_test_ball(app.world_mut(), Vec2::new(0.0, 0.0), Vec2::new(0.0, BALL_SPEED));

        app.add_systems(Update, paddle_powerup_collision);
        app.update();

        let sprite = app.world().entity(paddle).get::<Sprite>().unwrap();
        let expected_width = PADDLE_WIDTH * WIDE_PADDLE_MULTIPLIER;
        assert_eq!(
            sprite.custom_size,
            Some(Vec2::new(expected_width, PADDLE_HEIGHT)),
            "Paddle should be widened"
        );
    }

    #[test]
    fn multi_ball_spawns_two() {
        let mut app = test_app();
        spawn_test_paddle(app.world_mut(), 0.0);
        spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, BALL_SPEED),
        );
        // Spawn MultiBall power-up overlapping paddle
        app.world_mut().spawn((
            Sprite {
                custom_size: Some(Vec2::splat(POWERUP_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, PADDLE_Y, 0.0),
            PowerUp {
                power_type: PowerUpType::MultiBall,
            },
            Collider {
                size: Vec2::splat(POWERUP_SIZE),
            },
        ));

        app.add_systems(Update, paddle_powerup_collision);
        app.update();

        let ball_count = app
            .world_mut()
            .query::<&Ball>()
            .iter(app.world())
            .count();
        assert_eq!(ball_count, 3, "Should have 1 original + 2 extra balls");
    }

    #[test]
    fn slow_ball_reduces_speed() {
        let mut app = test_app();
        spawn_test_paddle(app.world_mut(), 0.0);
        let ball = spawn_test_ball(
            app.world_mut(),
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, BALL_SPEED),
        );
        app.world_mut().spawn((
            Sprite {
                custom_size: Some(Vec2::splat(POWERUP_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, PADDLE_Y, 0.0),
            PowerUp {
                power_type: PowerUpType::SlowBall,
            },
            Collider {
                size: Vec2::splat(POWERUP_SIZE),
            },
        ));

        app.add_systems(Update, paddle_powerup_collision);
        app.update();

        let vel = app.world().entity(ball).get::<Velocity>().unwrap();
        let expected_speed = BALL_SPEED * SLOW_BALL_MULTIPLIER;
        assert!(
            (vel.0.length() - expected_speed).abs() < 1.0,
            "Ball speed should be reduced: {} vs {}",
            vel.0.length(),
            expected_speed,
        );
    }

    #[test]
    fn powerup_effects_created() {
        let mut app = test_app();
        let paddle = spawn_test_paddle(app.world_mut(), 0.0);
        app.world_mut().spawn((
            Sprite {
                custom_size: Some(Vec2::splat(POWERUP_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, PADDLE_Y, 0.0),
            PowerUp {
                power_type: PowerUpType::WidePaddle,
            },
            Collider {
                size: Vec2::splat(POWERUP_SIZE),
            },
        ));
        spawn_test_ball(app.world_mut(), Vec2::new(0.0, 0.0), Vec2::new(0.0, BALL_SPEED));

        app.add_systems(Update, paddle_powerup_collision);
        app.update();
        // Apply commands
        app.update();

        let effects = app.world().entity(paddle).get::<PowerUpEffects>();
        assert!(effects.is_some(), "PowerUpEffects should be added to paddle");
        let effects = effects.unwrap();
        assert_eq!(effects.effects.len(), 1);
        assert_eq!(effects.effects[0].effect_type, PowerUpType::WidePaddle);
    }

    #[test]
    fn powerup_aabb_collision_works() {
        assert!(aabb_collision(
            Vec2::new(0.0, 0.0),
            Vec2::new(20.0, 20.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(20.0, 20.0),
        ));

        assert!(!aabb_collision(
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(10.0, 10.0),
        ));
    }
}
