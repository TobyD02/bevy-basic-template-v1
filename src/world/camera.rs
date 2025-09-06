use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Projection::from(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        })
    ));
}
