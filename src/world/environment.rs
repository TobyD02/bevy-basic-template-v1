use bevy::prelude::*;
use crate::animation_controller::animation_controller::AnimationControllerBundle;
use crate::animation_controller::animations::{GROUND_CORNER};
use crate::physics::body::Body;
use crate::physics::aabb::AABB;

pub fn spawn_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands
        .spawn((AnimationControllerBundle::new(
                vec![
                    GROUND_CORNER,
                ],
                asset_server,
                texture_atlas_layouts,
            ),
            Body::new(1, false),
            AABB::new(500., 100.),
            Transform::from_xyz(100., -200., 1.),
            GlobalTransform::default(),
        ));
}