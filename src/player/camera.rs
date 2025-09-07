use bevy::prelude::*;

#[derive(Bundle)]
pub struct CameraBundle {
    pub camera: Camera2d,
    pub projection: Projection,
}

impl CameraBundle {
    pub fn new () -> Self {
        Self {
            camera: Camera2d::default(),
            projection: Projection::from(OrthographicProjection {
                scale: 0.5,
                ..OrthographicProjection::default_2d()
            }),
        }
    }
}