use crate::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }

    pub fn hurt(&mut self, amount: f32) -> bool {
        self.current -= amount;
        if self.current.is_sign_negative() {
            self.current = 0.0;
            true
        } else {
            false
        }
    }

    pub fn can_be_heald(&self) -> bool {
        self.current < self.max
    }

    pub fn is_daed(&self) -> bool {
        self.current <= 0.0
    }

    pub fn heal(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }
}
