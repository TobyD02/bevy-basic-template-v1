use crate::components::{Health, HealthBundle};

use crate::entities::player::components::{Inventory, Player};
use crate::entities::player::systems::{player_movement};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, spawn_player))
            .add_systems(Update, player_movement);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        HealthBundle::new(100, 1.0),
        Inventory { items: vec![] },
        Sprite::from_image(
            asset_server.load("square.png")
        )
    ));
}
