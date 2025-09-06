use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldConstants {
    pub gravity: Vec2,
    pub min_speed: f32,
}

pub fn initialise_world_constants(mut commands: Commands) {
    commands.insert_resource(WorldConstants {
        gravity: Vec2::new(0., -981.),
        min_speed: 1.,
    });
}