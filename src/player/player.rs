use std::cmp::min;
use bevy::prelude::*;
use crate::animation_controller::animation_controller::{AnimationController, AnimationControllerBundle};
use crate::animation_controller::animations::{PLAYER_DEAD, PLAYER_IDLE, PLAYER_JUMP, PLAYER_RUN};
use crate::globals::{GRAVITY, MIN_SPEED, TILE_SIZE};
use crate::physics::aabb::AABB;
use crate::player::camera::CameraBundle;
use crate::physics::body::Body;

#[derive(Component, Debug)]
pub struct Player {
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
                    PLAYER_IDLE,
                    PLAYER_RUN,
                    PLAYER_JUMP,
                    PLAYER_DEAD,
                ],
                asset_server,
                texture_atlas_layouts,
            ),
            Body::new(0, true),
            AABB::new(TILE_SIZE.x as f32, TILE_SIZE.y as f32),
            Transform::from_xyz(0., 100., 0.),
            GlobalTransform::default(),
        ));
}

pub fn player_movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut AnimationController, &mut Body)>,
) {
    let dt = time.delta_secs();

    for (mut player, mut animation, mut body) in &mut query {
        // Set grounded state based on collision flags from the PREVIOUS frame.
        // This is crucial for stability.
        player.grounded = body.colliding_down;

        if player.grounded {
            player.jump_count = player.max_jump_count;
        } else {
            // Apply gravity only if not grounded.
            body.force += GRAVITY;
        }

        let mut input_dir: f32 = 0.;
        if input.pressed(KeyCode::KeyA) {
            input_dir -= player.accel;
        }
        if input.pressed(KeyCode::KeyD) {
            input_dir += player.accel;
        }

        let mut animation_name= "player_idle";
        // If no force is applied, apply friction_mod
        if input_dir == 0. {
            body.force.x -= body.velocity.x * player.friction_mod;
        } else {
            animation_name = "player_run";
            animation.flip_x = input_dir < 0.;
            body.force.x += input_dir;
        }

        if !player.grounded {
            animation_name = "player_jump";
        }

        animation.play_animation(animation_name);

        if input.just_pressed(KeyCode::Space) && player.jump_count > 0 {
            body.impulse.y += player.jump_force;
            player.jump_count -= 1;
            body.colliding_down = false;
        }

        if body.velocity.x.abs() > player.max_speed {
            body.velocity.x = body.velocity.x.signum() * player.max_speed;
        }
    }
}
