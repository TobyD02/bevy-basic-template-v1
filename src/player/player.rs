use std::cmp::min;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::animation_controller::animation_controller::{AnimationController, AnimationControllerBundle};
use crate::animation_controller::animations::{PLAYER_DEAD_ANIMATION, PLAYER_IDLE_ANIMATION, PLAYER_JUMP_ANIMATION, PLAYER_RUN_ANIMATION};
use crate::globals::{GRAVITY, MIN_SPEED};
use crate::player::camera::CameraBundle;

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
    fn new(max_speed: f32, accel: f32, jump_force: f32, friction_mod: f32, double_jump: bool) -> Self {
        let number_of_jumps = if double_jump { 2 } else { 1 };

        Self {
            max_speed,
            accel,
            jump_force,
            jump_count: number_of_jumps,
            max_jump_count: number_of_jumps,
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
        .spawn((
            Player::new(200., 1000., 350., 3.,true),
            CameraBundle::new(),
            AnimationControllerBundle::new(
                vec![
                    PLAYER_IDLE_ANIMATION,
                    PLAYER_RUN_ANIMATION,
                    PLAYER_JUMP_ANIMATION,
                    PLAYER_DEAD_ANIMATION,
                ],
                asset_server,
                texture_atlas_layouts,
            ),
            RigidBody::Dynamic,
            Velocity {
                linvel: Vec2::splat(0.),
                angvel: 0.,
            },
            LockedAxes::ROTATION_LOCKED,
            GravityScale{ 0: 0. },
            Restitution::new(0.),
            Ccd::enabled(),
            Collider::cuboid(8., 8.),
            KinematicCharacterController::default(),
            Transform::from_xyz(0., 0., 0.),
            GlobalTransform::default(),
        ));
}
pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut KinematicCharacterController, &mut Player, &mut AnimationController, &mut RigidBody, &mut Velocity)>,
) {
    let dt = time.delta_secs();


    for (mut controller, mut player, mut animation, mut rb, mut vel) in &mut query {
        let mut applied_force: Vec2 = (0., 0.).into();
        let mut applied_impulse: Vec2 = (0., 0.).into();
        let mut animation_block: bool = false;

        // ----------
        let translation = player.velocity * dt;
        controller.translation = Some(translation);


        if player.grounded {
            player.jump_count = player.max_jump_count;

            if player.velocity.y < 0.1 {
                player.velocity.y = 0.; // set y velocity to 0 when grounded
            }

        } else {
            player.jump_count = min(player.jump_count, 1);
            applied_force += GRAVITY; // apply gravity if not grounded
        }

        if input.just_pressed(KeyCode::Space) && player.jump_count > 0 {
            player.velocity.y = 0.;
            applied_impulse.y += player.jump_force;
            player.jump_count -= 1;
            animation.play_animation("player_jump");
            animation_block = true;
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

        vel.linvel = player.velocity * dt;
    }
}

pub fn update_player_state(mut query: Query<(&KinematicCharacterControllerOutput, &mut Player)>) {
    for (output, mut player) in &mut query {
        player.grounded = output.grounded;
    }
}