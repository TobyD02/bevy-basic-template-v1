use crate::world::world_constants::WorldConstants;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


#[derive(Component, Debug)]
pub struct Player {
    velocity: Vec2,
    max_speed: f32,
    accel: f32,
    grounded: bool,
    friction_mod: f32,
    jump_force: f32,
    jump_count: i32,
    max_jump_count: i32,
}

impl Player {
    fn new(max_speed: f32, accel: f32, jump_force: f32, friction_mod: f32) -> Self {
        Self {
            max_speed,
            accel,
            jump_force,
            jump_count: 1,
            max_jump_count: 1,
            friction_mod,
            velocity: Vec2::new(0., 0.),
            grounded: false,
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(5., 5.))
        .insert(Restitution::coefficient(0.0)) // no bounce
        .insert(Friction::coefficient(0.8)) // optional: add some friction
        .insert(KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            min_slope_slide_angle: 0.0_f32.to_radians(),
            offset: CharacterLength::Absolute(0.5),
            ..default()
        })
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert(Player::new(200., 1000., 200., 3.));
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    world_constants: Res<WorldConstants>,
    mut query: Query<(&mut KinematicCharacterController, &mut Player)>,
) {
    let dt = time.delta_secs();

    for (mut controller, mut player) in &mut query {
        let mut applied_force: Vec2 = (0., 0.).into();
        let mut applied_impulse: Vec2 = (0., 0.).into();

        if player.grounded {
            player.jump_count = player.max_jump_count;

            if player.velocity.y < 0. {
                player.velocity.y = -1.; // set y velocity to 0 when grounded
            }

            if input.pressed(KeyCode::Space) && player.jump_count > 0 {
                applied_impulse.y += player.jump_force;
                player.jump_count -= 1;
            }
        } else {
            applied_force += world_constants.gravity; // apply gravity if not grounded
        }

        let mut input_dir: f32 = 0.;
        if input.pressed(KeyCode::KeyA) {
            input_dir -= player.accel;
        }
        if input.pressed(KeyCode::KeyD) {
            input_dir += player.accel;
        }

        // If no force is applied, apply friction_mod
        if input_dir == 0. {
            applied_force.x -= player.velocity.x * player.friction_mod;
        } else {
            applied_force.x += input_dir;
        }

        player.velocity += applied_force * dt;
        player.velocity += applied_impulse;

        if player.velocity.x.abs() > player.max_speed {
            player.velocity.x = player.velocity.x.signum() * player.max_speed;
        }
        if player.velocity.x.abs() < world_constants.min_speed {
            player.velocity.x = 0.;
        }

        controller.translation = Some(player.velocity * dt);
    }
}

pub fn update_player_state(mut query: Query<(&KinematicCharacterControllerOutput, &mut Player)>) {
    for (output, mut player) in &mut query {
        player.grounded = output.grounded;
        // REMOVE THIS LINE: player.grounded = output.grounded;
        println!("{:?}", player);
    }
}
