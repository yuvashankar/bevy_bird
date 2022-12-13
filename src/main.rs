use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const HEIGHT: f32 = 1000.0;
pub const WIDTH: f32 = 1000.0;

pub const OBSTACLE_WIDTH: f32 = 50.0;
pub const SCROLL_SPEED: f32 = -100.0;
pub const IMPULSE: f32 = 25000.0;
pub const DENSITY: f32 = 50.0;
pub const GRAVITY_SCALE: f32 = 10.0;

const SPRITE_SIZE: f32 = 100.0;

const PLAYER_COLLISION_GROUP_ID: Group = Group::GROUP_1;
const WALL_OBSTACLE_GROUP_ID: Group = Group::GROUP_2;

const GAP_GROUP_ID: Group = Group::GROUP_3;

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
struct Player(f32);

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct InPlay;

struct GameOver(bool);

#[derive(Component)]
struct Score(u128);

#[derive(Component)]
pub struct CollisionFilters {
    pub player: CollisionGroups,
    pub wall: CollisionGroups,
    pub gap: CollisionGroups,
}

impl Default for CollisionFilters {
    fn default() -> Self {
        let player = CollisionGroups::new(
            PLAYER_COLLISION_GROUP_ID,
            WALL_OBSTACLE_GROUP_ID,
        );
        let wall = CollisionGroups::new(
            WALL_OBSTACLE_GROUP_ID,
            PLAYER_COLLISION_GROUP_ID,
        );
        let gap = CollisionGroups::new(GAP_GROUP_ID, PLAYER_COLLISION_GROUP_ID);

        Self { player, wall, gap }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Bird".to_string(),
            width: WIDTH,
            height: HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(setup_graphics)
        .add_startup_system(spawn_initial_ostacles)
        .add_system(spawn_timer_obstacles)
        .add_system(player_movement)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system(destroy_obstacles)
        .add_system(detect_collision)
        .add_system(detect_game_over)
        .add_system(display_intersection_info)
        .insert_resource(GameOver(false))
        .insert_resource(CollisionFilters::default())
        .insert_resource(Score(0))
        .init_resource::<SpawnNextObstacle>()
        .run();
}

struct SpawnNextObstacle {
    event_timer: Timer,
}

impl Default for SpawnNextObstacle {
    fn default() -> Self {
        SpawnNextObstacle {
            event_timer: Timer::from_seconds(3.0, true),
        }
    }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });
}

fn destroy_obstacles(
    mut commands: Commands,
    q: Query<(Entity, &Obstacle, &Transform)>,
) {
    for (e, _o, t) in q.iter() {
        if t.translation.x < -(WIDTH - 100.0) {
            commands.entity(e).despawn();
        }
    }
}
fn spawn_initial_ostacles(
    mut commands: Commands,
    collision_filters: Res<CollisionFilters>,
) {
    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(
            400.0, 500.0, 0.0,
        )))
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
        .insert(Velocity {
            linvel: Vec2::new(SCROLL_SPEED, 0.0),
            angvel: 0.0,
        })
        .insert(Obstacle)
        .insert(collision_filters.wall)
        .insert(InPlay);

    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(
            400.0, -500.0, 0.0,
        )))
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
        .insert(Velocity {
            linvel: Vec2::new(SCROLL_SPEED, 0.0),
            angvel: 0.0,
        })
        .insert(Obstacle)
        .insert(collision_filters.wall);
}

fn spawn_timer_obstacles(
    mut commands: Commands,
    mut timer: ResMut<SpawnNextObstacle>,
    time: Res<Time>,
    score: Res<Score>,
    collision_filters: Res<CollisionFilters>,
) {
    // Tick timer
    timer.event_timer.tick(time.delta());

    if timer.event_timer.just_finished() && score.0 != 0 {
        commands
            .spawn()
            .insert_bundle(SpatialBundle::from(Transform::from_xyz(
                400.0, 500.0, 0.0,
            )))
            .insert(RigidBody::KinematicVelocityBased)
            .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
            .insert(Velocity {
                linvel: Vec2::new(SCROLL_SPEED, 0.0),
                angvel: 0.0,
            })
            .insert(Obstacle)
            .insert(collision_filters.wall)
            .insert(InPlay);

        commands
            .spawn()
            .insert_bundle(SpatialBundle::from(Transform::from_xyz(
                400.0, -500.0, 0.0,
            )))
            .insert(RigidBody::KinematicVelocityBased)
            .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
            .insert(Velocity {
                linvel: Vec2::new(SCROLL_SPEED, 0.0),
                angvel: 0.0,
            })
            .insert(Obstacle)
            .insert(collision_filters.wall);
    }
}

fn spawn_player(
    mut commands: Commands,
    collision_filters: Res<CollisionFilters>,
) {
    // Spawn entity with `Player` struct as a component for access in movement query.
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(ExternalImpulse::default())
        .insert(Collider::ball(SPRITE_SIZE / 2.0))
        .insert(ColliderMassProperties::Density(DENSITY))
        .insert(GravityScale(GRAVITY_SCALE))
        .insert(Player(100.0))
        .insert(ActiveEvents::all())
        .insert(collision_filters.player);
}

fn display_intersection_info(
    mut commands: Commands,
    mut score: ResMut<Score>,
    obstacle_query: Query<(Entity, &Transform), With<InPlay>>,
) {
    for (entity, transform) in &obstacle_query {
        if transform.translation.x < (-SPRITE_SIZE) {
            score.0 += 1;
            info!("Passed obstacle, score: {}", score.0);

            commands.entity(entity).remove::<InPlay>();
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    game_over: Res<GameOver>,
    mut player_info: Query<&mut ExternalImpulse>,
) {
    if game_over.0 {
        return;
    }

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

fn detect_collision(
    mut game_over: ResMut<GameOver>,
    mut collision_event: EventReader<CollisionEvent>,
) {
    for event in collision_event.iter() {
        info!("Detected collision {:?}", event);
        game_over.0 = true;
    }
}

fn detect_game_over(game_over: Res<GameOver>) {
    if game_over.0 {
        // end game
    }
}
