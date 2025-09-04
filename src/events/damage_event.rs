use bevy::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub entity: Entity,
    pub amount: i32,
}

impl DamageEvent {
    pub fn new(entity: Entity, amount: i32) -> Self {
        Self {
            entity,
            amount,
        }
    }
}