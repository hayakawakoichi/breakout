use bevy::prelude::*;

use crate::components::*;

/// Move the ball based on its velocity
pub fn ball_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Ball>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn ball_moves_in_velocity_direction() {
        let mut app = test_app();
        let ball = spawn_test_ball(app.world_mut(), Vec2::ZERO, Vec2::new(300.0, 200.0));

        app.add_systems(Update, ball_movement);
        app.update();

        let transform = app.world().entity(ball).get::<Transform>().unwrap();
        assert!(transform.translation.x > 0.0, "Should move in +x");
        assert!(transform.translation.y > 0.0, "Should move in +y");
    }

    #[test]
    fn ball_moves_negative() {
        let mut app = test_app();
        let ball = spawn_test_ball(app.world_mut(), Vec2::ZERO, Vec2::new(-300.0, -200.0));

        app.add_systems(Update, ball_movement);
        app.update();

        let transform = app.world().entity(ball).get::<Transform>().unwrap();
        assert!(transform.translation.x < 0.0, "Should move in -x");
        assert!(transform.translation.y < 0.0, "Should move in -y");
    }

    #[test]
    fn zero_velocity_stays() {
        let mut app = test_app();
        let ball = spawn_test_ball(app.world_mut(), Vec2::new(50.0, 50.0), Vec2::ZERO);

        app.add_systems(Update, ball_movement);
        app.update();

        let transform = app.world().entity(ball).get::<Transform>().unwrap();
        assert!(
            (transform.translation.x - 50.0).abs() < f32::EPSILON,
            "Should not move with zero velocity"
        );
        assert!(
            (transform.translation.y - 50.0).abs() < f32::EPSILON,
            "Should not move with zero velocity"
        );
    }

    #[test]
    fn multiple_balls_independent() {
        let mut app = test_app();
        let ball_a = spawn_test_ball(app.world_mut(), Vec2::ZERO, Vec2::new(100.0, 0.0));
        let ball_b = spawn_test_ball(app.world_mut(), Vec2::ZERO, Vec2::new(0.0, 100.0));

        app.add_systems(Update, ball_movement);
        app.update();

        let ta = app.world().entity(ball_a).get::<Transform>().unwrap();
        let tb = app.world().entity(ball_b).get::<Transform>().unwrap();
        assert!(ta.translation.x > 0.0 && ta.translation.y.abs() < 0.01);
        assert!(tb.translation.y > 0.0 && tb.translation.x.abs() < 0.01);
    }
}
