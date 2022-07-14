use bevy::{prelude::*, core::Zeroable};
use bevy_rapier2d::prelude::*;

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
struct Player(f32);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Bird".to_string(),
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_system(player_movement)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

fn spawn_player(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

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
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce::default())
        .insert(Collider::ball(sprite_size / 2.0))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(GravityScale(1.0))
        .insert(Player(100.0));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut ExternalForce)>,
) {
    for (player, mut rb_forces) in player_info.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W)
            || keyboard_input.pressed(KeyCode::Up)
            || keyboard_input.pressed(KeyCode::Space);
        // let y_axis = up as i8;

        if up {
            rb_forces.force = Vec2::new(0.0, 100.0);
        } else {
            rb_forces.force = Vec2::ZERO;
        }
        
        // let mut move_delta = Vec2::new(0.0, y_axis as f32 * 1000.0);
        // if move_delta != Vec2::ZERO {
        //     move_delta /= move_delta.length();
        // }

        // // Update the velocity on the rigid_body_component,
        // // the bevy_rapier plugin will update the Sprite transform.
        // rb_vels.linvel = move_delta * player.0;
    }
}
