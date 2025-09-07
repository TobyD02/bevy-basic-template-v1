use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::globals::DEBUG_MODE;
use crate::world::camera::spawn_camera;
use crate::world::environment::spawn_environment;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_systems(Startup, (spawn_camera, spawn_environment));

        if DEBUG_MODE {

            app.add_plugins(RapierDebugRenderPlugin::default());
        }
    }
}