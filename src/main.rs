mod entities;
mod components;
mod systems;
mod events;

use bevy::prelude::*;
use crate::entities::player::plugin::PlayerPlugin;
use crate::events::{DamageEvent, EventsPlugin};
use crate::systems::SystemsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, EventsPlugin, SystemsPlugin))
        .run();
}