mod player;
mod world;
mod animation_controller;
mod globals;
mod ui;
mod physics;

use bevy::prelude::*;
use crate::animation_controller::plugin::AnimationControllerPlugin;
use crate::physics::plugin::PhysicsPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::ui::plugin::UiPlugin;
use crate::world::plugin::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1280., 720.).into(),
                        resizable: false,
                        ..default()
                    }),
                ..default()
                }
            ).set(
                ImagePlugin::default_nearest()
            )
        )
        .add_plugins((PlayerPlugin, WorldPlugin, AnimationControllerPlugin, UiPlugin, PhysicsPlugin))
        .run();
}