use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::world::camera::spawn_camera;
use crate::world::environment::spawn_environment;
use crate::world::world_constants::initialise_world_constants;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, (spawn_camera, spawn_environment, initialise_world_constants));
    }
}