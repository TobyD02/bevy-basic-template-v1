mod entities;
mod components;
mod systems;
mod events;

use bevy::prelude::*;
use crate::entities::EntitiesPlugin;
use crate::events::EventsPlugin;
use crate::systems::SystemsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((EntitiesPlugin, EventsPlugin, SystemsPlugin))
        .run();
}