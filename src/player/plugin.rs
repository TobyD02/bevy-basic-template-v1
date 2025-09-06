use bevy::prelude::*;
use crate::player::player::{spawn_player, update_player_state, player_movement};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement))
            .add_systems(PostUpdate, update_player_state);
    }
}