use crate::globals::MIN_SPEED;
use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Body {
    pub(crate) velocity: Vec2,
    pub(crate) force: Vec2,
    pub(crate) impulse: Vec2,
    pub(crate) next_translation: Vec3,
    pub(crate) z_index: i32,
    pub(crate) dynamic: bool,

    // collision flags
    pub(crate) colliding_left: bool,
    pub(crate) colliding_right: bool,
    pub(crate) colliding_up: bool,
    pub(crate) colliding_down: bool,
}

impl Body {
    pub fn new(z_index: i32, dynamic: bool) -> Self {
        Self {
            velocity: Vec2::ZERO,
            force: Vec2::ZERO,
            impulse: Vec2::ZERO,
            next_translation: Vec3::ZERO,
            z_index,
            dynamic,

            // collision flags
            colliding_left: false,
            colliding_right: false,
            colliding_up: false,
            colliding_down: false,
        }
    }
}

pub fn update_bodies(time: Res<Time>, mut query: Query<(&mut Body, &Transform)>) {
    let dt = time.delta_secs();
    for (mut body, transform) in &mut query {
        if !body.dynamic {
            continue;
        }

        let delta_v = (body.force * dt) + body.impulse;

        body.velocity += delta_v;
        body.force = Vec2::ZERO;
        body.impulse = Vec2::ZERO;

        if body.velocity.x.abs() < MIN_SPEED {
            body.velocity.x = 0.;
        }

        if body.velocity.y.abs() < MIN_SPEED {
            body.velocity.y = 0.;
        }

        println!("{:?}", body.colliding_down);
        // if body.colliding_down {
        //     body.velocity.y = 0.;
        // }

        let mut pos = transform.translation;
        pos += Vec3::new(
            body.velocity.x * dt,
            body.velocity.y * dt,
            body.z_index as f32,
        );

        body.next_translation = pos;
    }
}

pub fn move_bodies(mut query: Query<(&mut Body, &mut Transform)>) {
    for (mut body, mut transform) in &mut query {
        if !body.dynamic {
            continue;
        }

        // This is a crucial step. The collision flags from the previous frame's resolution
        // are used to decide whether to stop the body. However, as noted, this logic
        // is now handled by the `resolve_bodies` system. The physics pipeline
        // will now be cleaner and more stable.

        // println!("{:?}", body.colliding_down);
        if body.colliding_down && body.velocity.y < 0. {
            body.velocity.y = 0.;
        }


        transform.translation = body.next_translation;
        body.next_translation = Vec3::ZERO;
    }
}
