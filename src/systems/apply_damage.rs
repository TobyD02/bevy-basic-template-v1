use bevy::prelude::*;
use crate::components::core::{Cooldown, Health};
use crate::events::DamageEvent;

pub fn apply_damage(
    mut events: EventReader<DamageEvent>,
    mut query: Query<(&mut Health, &mut Cooldown)>,
) {
    for event in events.read() {
        if let Ok((mut health, mut cooldown)) = query.get_mut(event.entity) {
            if cooldown.finished() {
                health.take_damage(event.amount);
                cooldown.reset();
                println!("{} took {} damage. health is now {}", event.entity, event.amount, health.current_hp);
            }
        }
    }
}