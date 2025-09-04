use bevy::prelude::*;
use crate::events::DamageEvent;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>();
    }
}
