use bevy::prelude::*;
use crate::entities::player::components::Player;
use crate::events::DamageEvent;

pub fn player_movement(
    mut damage_writer: EventWriter<DamageEvent>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut Transform), With<Player>>,
) {
    for (entity, mut transform) in &mut query {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Space) {
            damage_writer.write(DamageEvent::new(entity, 10));
        }

        let speed = 100.0f32;

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * speed * time.delta_secs();
        }
    }
}