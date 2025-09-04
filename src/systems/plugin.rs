use bevy::prelude::*;
use crate::systems::{apply_damage, cooldown_tick};

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (apply_damage, cooldown_tick));
    }
}