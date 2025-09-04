use bevy::prelude::Component;

#[derive(Component)]
pub struct Cooldown {
    max: f32,
    current: f32,
}

impl Cooldown {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }

    pub fn reset(&mut self) {
        self.current = self.max;
    }

    pub fn finished(&self) -> bool {
        self.current <= 0.0
    }

    pub fn tick(&mut self, delta: f32) {
        if self.current > 0.0 {
            self.current -= delta
        } else {
            self.current = 0.0;
        }
    }
}