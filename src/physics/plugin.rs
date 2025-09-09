use bevy::prelude::*;
use crate::physics::aabb::{reset_collision_flags, resolve_bodies};
use crate::physics::body::{move_bodies, update_bodies};
use crate::player::player::player_movement;
#[cfg(feature="debug")]
use crate::physics::aabb::draw_debug_colliders;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Reset collision flags at the very beginning of the frame.
            // Player input and force application should happen in the `Update` stage,
            // so it can read the collision state from the last physics step.
            .add_systems(Update, (player_movement))

            // The `PostUpdate` stage is where the main physics pipeline will run.
            .add_systems(PostUpdate, (
                // 1. Update the body's position based on velocity and forces.
                update_bodies.after(reset_collision_flags),
                // 2. Resolve collisions and correct the body's position and velocity.
                resolve_bodies.after(update_bodies),
                // 3. Finally, apply the resolved position to the transform.
                move_bodies.after(resolve_bodies)
            ));


        #[cfg(feature="debug")]
        app.add_systems(Update, draw_debug_colliders);
    }
}
