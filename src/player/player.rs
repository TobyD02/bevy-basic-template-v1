use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::animation_controller::animation_controller::{AnimationController, AnimationControllerBundle};
use crate::animation_controller::animations::{PLAYER_DEAD_ANIMATION, PLAYER_IDLE_ANIMATION, PLAYER_JUMP_ANIMATION, PLAYER_RUN_ANIMATION};
use crate::globals::{GRAVITY, MIN_SPEED};

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

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(8., 8.))
        // .insert(Restitution::coefficient(0.0)) // no bounce
        // .insert(Friction::coefficient(0.8)) // optional: add some friction
        .insert(KinematicCharacterController {
            offset: CharacterLength::Absolute(1.),
            snap_to_ground: Some(CharacterLength::Absolute( 2.)),
            ..default()
        })
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert(Player::new(200., 1000., 200., 3.))
        .insert(AnimationControllerBundle::new(
                vec!(PLAYER_IDLE_ANIMATION, PLAYER_RUN_ANIMATION, PLAYER_JUMP_ANIMATION, PLAYER_DEAD_ANIMATION),
                asset_server,
                texture_atlas_layouts
            )
        );
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &mut Player, &mut AnimationController)>,
) {
    let dt = time.delta_secs();

    for (mut controller, mut player, mut animation) in &mut query {
        let mut applied_force: Vec2 = (0., 0.).into();
        let mut applied_impulse: Vec2 = (0., 0.).into();
        let mut animation_block: bool = false;

        if player.grounded {
            player.jump_count = player.max_jump_count;

            if player.velocity.y < 0. {
                player.velocity.y = 0.; // set y velocity to 0 when grounded
            }

            if input.pressed(KeyCode::Space) && player.jump_count > 0 {
                applied_impulse.y += player.jump_force;
                player.jump_count -= 1;
                animation.play_animation("player_jump");
                animation_block = true;
            }
        } else {
            applied_force += GRAVITY; // apply gravity if not grounded
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
            if !animation_block & player.grounded{
                animation.play_animation("player_idle");
            }
            applied_force.x -= player.velocity.x * player.friction_mod;
        } else {
            if !animation_block & player.grounded {
                animation.play_animation("player_run");
            }
            animation.flip_x = input_dir < 0.;
            applied_force.x += input_dir;
        }

        player.velocity += applied_force * dt;
        player.velocity += applied_impulse;

        if player.velocity.x.abs() > player.max_speed {
            player.velocity.x = player.velocity.x.signum() * player.max_speed;
        }
        if player.velocity.x.abs() < MIN_SPEED {
            player.velocity.x = 0.;
        }

        controller.translation = Some(player.velocity * dt);
    }
}

pub fn update_player_state(mut query: Query<(&KinematicCharacterControllerOutput, &mut Player)>) {
    for (output, mut player) in &mut query {
        player.grounded = output.grounded;
    }
}
