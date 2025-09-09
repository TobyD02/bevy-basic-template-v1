use bevy::prelude::*;
use crate::world::environment::spawn_environment;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_environment);
    }
}