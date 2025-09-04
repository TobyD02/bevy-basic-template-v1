use bevy::prelude::*;
use crate::components::core::Cooldown;

pub fn cooldown_tick(
    time: Res<Time>,
    mut query: Query<&mut Cooldown>
) {
    for mut cooldown in &mut query {

        cooldown.tick(time.delta_secs());
    }
}