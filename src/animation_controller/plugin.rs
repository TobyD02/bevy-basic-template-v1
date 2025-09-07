use bevy::prelude::*;
use crate::animation_controller::animation_controller::animate_sprite;

pub struct AnimationControllerPlugin;

impl Plugin for AnimationControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite);
    }
}