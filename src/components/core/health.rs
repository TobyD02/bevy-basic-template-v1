use bevy::prelude::*;

#[derive(Component)]
pub struct Health{
    pub max_hp: i32,
    pub current_hp: i32,
}

impl Health {
    pub fn new(max_hp: i32) -> Self {
        Self {
            max_hp,
            current_hp: max_hp,
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.current_hp -= amount;

        if self.current_hp <= 0 {
            self.current_hp = 0;
        }
    }
}
