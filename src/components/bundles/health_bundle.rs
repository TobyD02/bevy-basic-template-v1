use bevy::prelude::*;
use crate::components::core::{Cooldown, Health};

#[derive(Bundle)]
pub struct HealthBundle {
    pub health: Health,
    pub cooldown: Cooldown
}

impl HealthBundle {
    pub fn new(max_hp: i32, max_cooldown: f32) -> Self {
        Self {health: Health::new(max_hp), cooldown: Cooldown::new(max_cooldown)}
    }
}