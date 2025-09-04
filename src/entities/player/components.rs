use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<String>,
}
