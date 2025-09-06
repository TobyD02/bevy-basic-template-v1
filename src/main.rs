mod player;
mod world;

use bevy::prelude::*;
use crate::player::plugin::PlayerPlugin;
use crate::world::plugin::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1280., 720.).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins((PlayerPlugin, WorldPlugin))
        .run();
}