use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::states::GameState;
use crate::systems::audio::CollisionEvent;
use crate::systems::effects::TrailTimer;

/// Create a minimal Bevy App for testing with all resources and events registered.
/// Includes a bootstrap frame (first update always has delta=0).
pub fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(StatesPlugin);
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        std::time::Duration::from_secs_f64(1.0 / 60.0),
    ));
    app.init_resource::<Score>();
    app.init_resource::<Level>();
    app.init_resource::<ScreenShake>();
    app.init_resource::<ComboTracker>();
    app.insert_resource(HighScores { scores: [0, 0, 0] });
    app.init_resource::<LevelStats>();
    app.init_resource::<TrailTimer>();
    app.init_state::<GameState>();
    app.add_event::<CollisionEvent>();
    // Bootstrap frame: first update always produces delta=0
    app.update();
    app
}

pub fn spawn_test_paddle(world: &mut World, x: f32) -> Entity {
    world
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x, PADDLE_Y, 0.0),
            Paddle,
            Collider {
                size: Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT),
            },
        ))
        .id()
}

pub fn spawn_test_ball(world: &mut World, pos: Vec2, vel: Vec2) -> Entity {
    world
        .spawn((
            Sprite {
                custom_size: Some(Vec2::new(BALL_SIZE, BALL_SIZE)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
            Ball,
            Velocity(vel),
            Collider {
                size: Vec2::new(BALL_SIZE, BALL_SIZE),
            },
        ))
        .id()
}

pub fn spawn_test_block(world: &mut World, pos: Vec2) -> Entity {
    world
        .spawn((
            Sprite {
                color: Color::srgb(0.92, 0.46, 0.46),
                custom_size: Some(Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
            Block {
                block_type: BlockType::Normal,
            },
            Collider {
                size: Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT),
            },
        ))
        .id()
}

pub fn spawn_test_block_typed(world: &mut World, pos: Vec2, block_type: BlockType) -> Entity {
    world
        .spawn((
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
            Block { block_type },
            Collider {
                size: Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT),
            },
        ))
        .id()
}

pub fn spawn_test_wall(world: &mut World, wall_type: Wall, pos: Vec2, size: Vec2) -> Entity {
    world
        .spawn((
            Sprite::default(),
            Transform::from_xyz(pos.x, pos.y, 0.0),
            wall_type,
            Collider { size },
        ))
        .id()
}

/// Resource to collect collision events for test assertions
#[derive(Resource, Default)]
pub struct CollectedEvents {
    pub events: Vec<CollisionEvent>,
}

/// System to collect collision events into the CollectedEvents resource
pub fn collect_collision_events(
    mut events: EventReader<CollisionEvent>,
    mut collected: ResMut<CollectedEvents>,
) {
    for event in events.read() {
        collected.events.push(event.clone());
    }
}
