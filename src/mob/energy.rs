use bevy::reflect;

use crate::prelude::*;

use super::{
    diet::Stomach,
    health::{self, HealthEvent},
};

#[derive(Reflect)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
}

impl Default for Energy {
    fn default() -> Self {
        Self {
            current: 50.0,
            max: 50.0,
        }
    }
}

impl Energy {
    fn is_maxed(&self) -> bool {
        self.current >= self.max
    }

    pub fn add(&mut self, amount: f32) -> Option<f32> {
        debug_assert_eq!(amount.is_sign_positive(), true); // this should only be positive
        self.current += amount;
        self.fix_max()
    }

    pub fn sub(&mut self, amount: f32) -> Option<f32> {
        debug_assert_eq!(amount.is_sign_positive(), true); // tgus should only be negitive
        self.current -= amount;
        if self.current < 0.0 {
            let out = self.current.abs();
            self.current = 0.0;
            Some(out)
        } else {
            None
        }
    }

    fn fix_max(&mut self) -> Option<f32> {
        if self.is_maxed() {
            let out = self.current - self.max;
            self.current = self.max;
            Some(out)
        } else {
            None
        }
    }

    pub fn percent(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Reflect)]
pub struct Stored {
    pub current: f32,
    pub max: f32,
}

impl Default for Stored {
    fn default() -> Self {
        Self {
            current: 50.0,
            max: 50.0,
        }
    }
}

impl Stored {
    fn is_maxed(&self) -> bool {
        self.current >= self.max
    }

    pub fn add(&mut self, amount: f32) {
        debug_assert_eq!(amount.is_sign_positive(), true); // this should only be positive
        self.current += amount;
        if self.is_maxed() {
            self.current = self.max;
        }
    }

    pub fn sub(&mut self, amount: f32) -> Option<f32> {
        debug_assert_eq!(amount.is_sign_positive(), true); // tgus should only be negitive
        self.current -= amount;
        if self.current < 0.0 {
            let o = Some(self.current.abs());
            self.current = 0.0;
            o
        } else {
            None
        }
    }

    pub fn percent(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Stamina {
    pub energy: Energy,
    pub stored: Stored,
    pub min_energy: f32,
    pub convertion_rate_to: f32,
    pub convertion_rate_from: f32,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            energy: Energy::default(),
            stored: Stored::default(),
            min_energy: 0.25,
            convertion_rate_from: 1.0,
            convertion_rate_to: 0.5,
        }
    }
}

impl Stamina {
    pub fn add(&mut self, amount: f32) {
        if let Some(val) = self.energy.add(amount) {
            self.stored.add(val * self.convertion_rate_to);
        }
    }

    // Some(Damage)
    pub fn sub(&mut self, amount: f32) -> Option<f32> {
        if let Some(amount) = self.energy.sub(amount) {
            self.stored.sub(amount * self.convertion_rate_from)
        } else {
            None
        }
    }

    pub fn is_starving(&self) -> bool {
        self.stored.percent() <= self.min_energy
    }
}

pub enum StaminaEvent {
    Gain(f32, Entity),
    Lose(f32, Entity),
}

pub fn stamina_event_handler(
    mut events: EventReader<StaminaEvent>,
    mut query: Query<&mut Stamina>,
    mut health: EventWriter<HealthEvent>,
) {
    use StaminaEvent::*;
    for event in events.iter() {
        match event {
            Gain(amount, e) => {
                if let Ok(mut stamina) = query.get_mut(*e) {
                    stamina.add(*amount)
                }
            }
            Lose(amount, e) => {
                if let Ok(mut stamina) = query.get_mut(*e) {
                    if let Some(hurt) = stamina.sub(*amount) {
                        health.send(HealthEvent::Harm(hurt, *e))
                    }
                }
            }
        }
    }
}
