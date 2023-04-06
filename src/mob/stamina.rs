use crate::prelude::*;

#[derive(Component)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
        }
    }
}

impl Stamina {
    pub fn add(&mut self, amount: f32) {
        self.current += amount;
        if self.current > self.max {
            self.current = self.max;
        }
    }

    pub fn sub(&mut self, amount: f32) -> bool {
        self.current -= amount;
        if self.current.is_sign_negative() {
            self.current = 0.0;
            true
        } else {
            false
        }
    }

    pub fn percent(&self) -> f32 {
        self.current / self.max * 100.0
    }

    pub fn has_at_least(&self, ammount: f32) -> bool {
        self.current > ammount
    }
}
