use bevy::input::ButtonState;
use bevy::sprite::Anchor;
use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use std::path::Path;

pub const HEIGHT: f32 = 1000.0;
pub const WIDTH: f32 = 500.0;

pub const OBSTACLE_WIDTH: f32 = 50.0;
pub const SCROLL_SPEED: f32 = -100.0;
pub const IMPULSE: f32 = 25000.0;
pub const DENSITY: f32 = 50.0;
pub const GRAVITY_SCALE: f32 = 10.0;

const SPRITE_SIZE: f32 = 100.0;

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct InPlay;
struct GameOver(bool);

#[derive(Component)]
struct Score(u128);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    InGame,
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
        .add_startup_system(setup_graphics)
        .add_startup_system(setup)
        .add_system(infinite_scroll)
        .add_state(AppState::Menu)
        .add_system_set(
            SystemSet::on_update(AppState::Menu).with_system(start_menu),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(spawn_player),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(player_movement),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_initial_ostacles),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(spawn_timer_obstacles),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(destroy_obstacles),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(detect_collision),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(detect_game_over),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(display_intersection_info),
        )
        // Debug
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // Resources
        .insert_resource(GameOver(false))
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

#[derive(Component)]
struct ActionKey(KeyCode);
fn start_menu(
    mut cmds: Commands,
    mut state: ResMut<State<AppState>>,
    mut key_evr: EventReader<KeyboardInput>,
) {
    for ev in &mut key_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {}
            ButtonState::Released => {
                info!("Action key is: : {:?} ({})", ev.key_code, ev.scan_code);
                if let Some(key_code) = ev.key_code {
                    state.set(AppState::InGame).unwrap();
                    cmds.insert_resource(ActionKey(key_code));
                }
            }
        }
    }
}

#[derive(Component)]
struct Background;

fn setup_graphics(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_path = Path::new("textures");
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 50.0),
        ..default()
    });

    // Spawn Background
    commands
        .spawn_bundle(SpriteBundle {
            // transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(6.8 * WIDTH, 1.2 * HEIGHT)),
                ..default()
            },
            texture: asset_server.load(texture_path.join("background.png")),
            ..default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity {
            linvel: Vec2::new(0.1 * SCROLL_SPEED, 0.0),
            ..default()
        })
        .insert(Background);

    // Spawn the next background plane
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(6.8 * WIDTH, 0.0, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(6.8 * WIDTH, 1.2 * HEIGHT)),
                ..default()
            },
            texture: asset_server.load(texture_path.join("background.png")),
            ..default()
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity {
            linvel: Vec2::new(0.1 * SCROLL_SPEED, 0.0),
            ..default()
        })
        .insert(Background);
}

/// Constantly spawns and despawns background textures as they come in and out
/// of camera view
fn infinite_scroll(
    mut cmds: Commands,
    background_query: Query<(Entity, &Transform), With<Background>>,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform) in &background_query {
        if transform.translation.x < -6.8 * WIDTH {
            let texture_path = Path::new("textures");
            info!("Despawning background text");
            cmds.entity(entity).despawn();
            info!("Spawning Next background");
            // Spawn Background
            cmds.spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(6.8 * WIDTH - 2.0, 0.0, 0.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(6.8 * WIDTH, 1.2 * HEIGHT)),
                    ..default()
                },
                texture: asset_server.load(texture_path.join("background.png")),
                ..default()
            })
            .insert(RigidBody::KinematicVelocityBased)
            .insert(Velocity {
                linvel: Vec2::new(0.1 * SCROLL_SPEED, 0.0),
                ..default()
            })
            .insert(Background);
        }
    }
}

fn spawn_initial_ostacles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_path = Path::new("textures");

    // Top Obstacle
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                flip_y: true,
                custom_size: Some(Vec2::new(OBSTACLE_WIDTH * 2.0, 300.0)),
                anchor: Anchor::TopCenter,
                ..default()
            },
            // Top Collider Transform
            transform: Transform::from_xyz(400.0, 500.0, 0.0),
            texture: asset_server.load(texture_path.join("obstacle3.png")),
            ..default()
        })
        // Top Obstacle Textures
        .with_children(|children| {
            // Middle Texture
            children.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    flip_y: true,
                    custom_size: Some(Vec2::new(
                        OBSTACLE_WIDTH * 1.6,
                        OBSTACLE_WIDTH * 3.2,
                    )),
                    ..default()
                },
                texture: asset_server.load(texture_path.join("obstacle2.png")),
                transform: Transform::from_xyz(0.0, 70.0, 0.0),
                ..default()
            });
            // Bottom Texture
            children.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    flip_y: true,
                    custom_size: Some(Vec2::new(
                        OBSTACLE_WIDTH * 1.6,
                        OBSTACLE_WIDTH * 3.2,
                    )),
                    ..default()
                },
                texture: asset_server.load(texture_path.join("obstacle2.png")),
                transform: Transform::from_xyz(0.0, 70.0 * 2.0, 0.0),
                ..default()
            });
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
        .insert(Velocity {
            linvel: Vec2::new(SCROLL_SPEED, 0.0),
            angvel: 0.0,
        })
        .insert(ActiveCollisionTypes::all())
        .insert(Obstacle)
        .insert(InPlay);

    // Bottom Obstacle
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                //  flip_y: true,
                custom_size: Some(Vec2::new(OBSTACLE_WIDTH * 2.0, 300.0)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            // Bottom Collider Transform
            transform: Transform::from_xyz(400.0, -500.0, 0.0),
            texture: asset_server.load(texture_path.join("obstacle3.png")),
            ..default()
        })
        // Bottom Obstacle Textures
        .with_children(|children| {
            children.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    flip_y: true,
                    custom_size: Some(Vec2::new(
                        OBSTACLE_WIDTH * 1.6,
                        OBSTACLE_WIDTH * 3.2,
                    )),
                    ..default()
                },
                texture: asset_server.load(texture_path.join("obstacle2.png")),
                transform: Transform::from_xyz(0.0, -70.0, 0.0),
                ..default()
            });

            // Bottom Texture
            children.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    flip_y: true,
                    custom_size: Some(Vec2::new(
                        OBSTACLE_WIDTH * 1.6,
                        OBSTACLE_WIDTH * 3.2,
                    )),
                    ..default()
                },
                texture: asset_server.load(texture_path.join("obstacle2.png")),
                transform: Transform::from_xyz(0.0, -70.0 * 2.0, 0.0),
                ..default()
            });
        })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
        .insert(Velocity {
            linvel: Vec2::new(SCROLL_SPEED, 0.0),
            angvel: 0.0,
        })
        .insert(ActiveCollisionTypes::all())
        .insert(Obstacle);

    // Floor Collider
    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(
            0.0, -800.0, 0.0,
        )))
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(600.0, 100.0))
        .insert(ActiveCollisionTypes::all())
        .insert(Obstacle);

    // Ceiling Collider
    commands
        .spawn()
        .insert_bundle(SpatialBundle::from(Transform::from_xyz(
            0.0, 800.0, 0.0,
        )))
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(600.0, 100.0))
        .insert(ActiveCollisionTypes::all())
        .insert(Obstacle);
}

fn destroy_obstacles(
    mut commands: Commands,
    q: Query<(Entity, &Obstacle, &Transform)>,
) {
    for (e, _o, t) in q.iter() {
        if t.translation.x < -(WIDTH - 100.0) {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn spawn_timer_obstacles(
    mut commands: Commands,
    mut timer: ResMut<SpawnNextObstacle>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let texture_path = Path::new("textures");
    // Tick timer
    timer.event_timer.tick(time.delta());

    if timer.event_timer.just_finished() && score.0 != 0 {
        let rng_val = rand::thread_rng().gen_range(-1.0_f32..1.0_f32);

        let offset = 150.0 * rng_val; // Randomly shift obstacles to add variety.

        let squeeze_offset = 110.0 * (score.0 as f32 / 10.0); // Decrease the gap as the score increases
        dbg!(squeeze_offset);

        let top_obstacle_y = 500.0 + offset - squeeze_offset;
        let bottom_obstacle_y = -500.0 + offset + squeeze_offset;

        // Top Obstacle
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    flip_y: true,
                    custom_size: Some(Vec2::new(OBSTACLE_WIDTH * 2.0, 300.0)),
                    anchor: Anchor::TopCenter,
                    ..default()
                },
                // Top Collider Transform
                transform: Transform::from_xyz(400.0, top_obstacle_y, 0.0),
                texture: asset_server.load(texture_path.join("obstacle3.png")),
                ..default()
            })
            // Top Obstacle Extension
            .with_children(|children| {
                // Middle Texture
                children.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        flip_y: true,
                        custom_size: Some(Vec2::new(
                            OBSTACLE_WIDTH * 1.6,
                            OBSTACLE_WIDTH * 3.2,
                        )),
                        ..default()
                    },
                    texture: asset_server
                        .load(texture_path.join("obstacle2.png")),
                    transform: Transform::from_xyz(0.0, 70.0, 0.0),
                    ..default()
                });

                // Bottom Texture
                children.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        flip_y: true,
                        custom_size: Some(Vec2::new(
                            OBSTACLE_WIDTH * 1.6,
                            OBSTACLE_WIDTH * 3.2,
                        )),
                        ..default()
                    },
                    texture: asset_server
                        .load(texture_path.join("obstacle2.png")),
                    transform: Transform::from_xyz(0.0, 70.0 * 2.0, 0.0),
                    ..default()
                });
            })
            .insert(RigidBody::KinematicVelocityBased)
            .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
            .insert(Velocity {
                linvel: Vec2::new(SCROLL_SPEED, 0.0),
                angvel: 0.0,
            })
            .insert(Obstacle)
            .insert(ActiveCollisionTypes::all())
            .insert(InPlay);

        // Bottom Obstacle
        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    //  flip_y: true,
                    custom_size: Some(Vec2::new(OBSTACLE_WIDTH * 2.0, 300.0)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                // Bottom Collider Transform
                transform: Transform::from_xyz(400.0, bottom_obstacle_y, 0.0),
                texture: asset_server.load(texture_path.join("obstacle3.png")),
                ..default()
            })
            // Top Obstacle Extension
            .with_children(|children| {
                // Middle Texture
                children.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        flip_y: true,
                        custom_size: Some(Vec2::new(
                            OBSTACLE_WIDTH * 1.6,
                            OBSTACLE_WIDTH * 3.2,
                        )),
                        ..default()
                    },
                    texture: asset_server
                        .load(texture_path.join("obstacle2.png")),
                    transform: Transform::from_xyz(0.0, -70.0, 0.0),
                    ..default()
                });
                // Bottom Texture
                children.spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        flip_y: true,
                        custom_size: Some(Vec2::new(
                            OBSTACLE_WIDTH * 1.6,
                            OBSTACLE_WIDTH * 3.2,
                        )),
                        ..default()
                    },
                    texture: asset_server
                        .load(texture_path.join("obstacle2.png")),
                    transform: Transform::from_xyz(0.0, -70.0 * 2.0, 0.0),
                    ..default()
                });
            })
            .insert(RigidBody::KinematicVelocityBased)
            .insert(Collider::cuboid(OBSTACLE_WIDTH, 300.0))
            .insert(Velocity {
                linvel: Vec2::new(SCROLL_SPEED, 0.0),
                angvel: 0.0,
            })
            .insert(ActiveCollisionTypes::all())
            .insert(Obstacle);
    }
}

fn spawn_player(
    mut commands: Commands,
    text_query: Query<Entity, With<WelcomeText>>,
    asset_server: Res<AssetServer>,
) {
    let texture_path = Path::new("textures");
    // Spawn entity with `Player` struct as a component for access in movement query.
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load(texture_path.join("bevy.png")),
            sprite: Sprite {
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
        .insert(Player)
        .insert(ActiveEvents::all());

    for entity in &text_query {
        commands.entity(entity).despawn();
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    action_key: Res<ActionKey>,
    game_over: Res<GameOver>,
    mut player_info: Query<&mut ExternalImpulse>,
) {
    if game_over.0 {
        return;
    }

    // let mut rb_impulse = player_info.single_mut();
    for mut player in &mut player_info {
        let up = keyboard_input.just_pressed(KeyCode::W)
            || keyboard_input.just_pressed(KeyCode::Up)
            || keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(action_key.0);

        if up {
            player.impulse = Vec2::new(0.0, IMPULSE);
        } else {
            player.impulse = Vec2::ZERO;
        }
    }
}

fn display_intersection_info(
    mut commands: Commands,
    mut score: ResMut<Score>,
    game_over: Res<GameOver>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    obstacle_query: Query<(Entity, &Transform), With<InPlay>>,
) {
    if game_over.0 {
        return;
    }

    for (entity, transform) in &obstacle_query {
        if transform.translation.x < (-SPRITE_SIZE) {
            score.0 += 1;
            info!("Passed obstacle, score: {}", score.0);

            commands.entity(entity).remove::<InPlay>();
        }
    }

    for mut text in &mut text_query {
        text.sections[1].value = format!("{}", score.0);
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

fn detect_game_over(
    mut commands: Commands,
    game_over: Res<GameOver>,
    asset_server: Res<AssetServer>,
) {
    if game_over.0 {
        let fonts_path = Path::new("fonts");
        commands.spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Oof, RIP.",
                TextStyle {
                    font: asset_server
                        .load(fonts_path.join("FiraSans-Bold.ttf")),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        );
    }
}

#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct WelcomeText;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fonts_path = Path::new("fonts");
    // Score Text
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: asset_server
                            .load(fonts_path.join("FiraSans-Bold.ttf")),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server
                        .load(fonts_path.join("FiraMono-Medium.ttf")),
                    font_size: 60.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(ScoreText);

    // Start Text
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "Choose your \naction button!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 80.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(440.0),
                    right: Val::Px(25.0 + 12.5),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(WelcomeText);
}
