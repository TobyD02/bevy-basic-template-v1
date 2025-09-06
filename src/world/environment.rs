use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn spawn_environment(mut commands: Commands) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 50.0))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));
}