use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const HEIGHT: f32 = 1000.0;
pub const WIDTH: f32 = 1000.0;

pub const OBSTACLE_WIDTH: f32 = 50.0;
pub const SCROLL_SPEED: f32 = -100.0;
pub const IMPULSE: f32 = 25000.0;
pub const DENSITY: f32 = 50.0;
pub const GRAVITY_SCALE: f32 = 10.0;
// The float value is the player movement speed in 'pixels/second'.

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Bevy Bird".to_string(),
        width: WIDTH,
        height: HEIGHT,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_player)
    .add_startup_system(setup_graphics)
    .add_system(player_movement)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(spawn_ostacles)
    .add_system(destroy_obstacles);

    app
}

#[derive(Component)]
struct Player(f32);

#[derive(Component)]
struct Obstacle;

fn setup_graphics(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });
}

fn destroy_obstacles(
    mut commands: Commands,
    q: Query<(Entity, &Obstacle, &Transform)>,
) {
    for (e, o, t) in q.iter() {
        if t.translation.x < -(WIDTH - 100.0) {
            commands.entity(e).despawn_recursive();
        }
    }
}
fn spawn_ostacles(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            400.0, 500.0, 0.0,
        )))
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
        .insert(Velocity {
            linvel: Vec2::new(SCROLL_SPEED, 0.0),
            angvel: 0.0,
        })
        .insert(Obstacle);

    commands
        .spawn()
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            400.0, -500.0, 0.0,
        )))
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
        .insert(Velocity {
            linvel: Vec2::new(SCROLL_SPEED, 0.0),
            angvel: 0.0,
        })
        .insert(Obstacle);
}

fn spawn_player(mut commands: Commands) {
    commands.spawn();

    let sprite_size = 100.0;

    // Spawn entity with `Player` struct as a component for access in movement query.
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                ..Default::default()
            },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(ExternalImpulse::default())
        .insert(Collider::ball(sprite_size / 2.0))
        .insert(ColliderMassProperties::Density(DENSITY))
        .insert(GravityScale(GRAVITY_SCALE))
        .insert(Player(100.0));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<&mut ExternalImpulse>,
) {
    let mut rb_impulse = player_info.single_mut();
    let up = keyboard_input.just_pressed(KeyCode::W)
        || keyboard_input.just_pressed(KeyCode::Up)
        || keyboard_input.just_pressed(KeyCode::Space);

    if up {
        rb_impulse.impulse = Vec2::new(0.0, IMPULSE);
    } else {
        rb_impulse.impulse = Vec2::ZERO;
    }
}
