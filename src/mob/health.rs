use crate::prelude::*;

use super::{
    death::DeathEvent,
    energy::{Stamina, StaminaEvent},
};

#[derive(Component, Reflect)]
pub struct Health {
    max: f32,
    current: f32,
}

impl Health {
    pub fn is_full(&self) -> bool {
        self.current >= self.max
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            max: 100.0,
            current: 50.0,
        }
    }
}

pub enum HealthEvent {
    Heal(f32, Entity),
    Harm(f32, Entity),
    Kill(Entity),
}

pub fn health_event_handler(
    mut query: Query<&mut Health>,
    mut events: EventReader<HealthEvent>,
    mut kill: EventWriter<DeathEvent>,
) {
    for event in events.iter() {
        match *event {
            HealthEvent::Heal(amount, e) => {
                if let Ok(mut health) = query.get_mut(e) {
                    health.current += amount;
                    if health.current > health.max {
                        health.current = health.max;
                    }
                }
            }
            HealthEvent::Harm(amount, e) => {
                if let Ok(mut health) = query.get_mut(e) {
                    health.current -= amount;
                    if health.current < 0.0 {
                        kill.send(DeathEvent(e));
                    }
                }
            }

            HealthEvent::Kill(e) => {
                kill.send(DeathEvent(e));
            }
        }
    }
}

#[derive(Component, Reflect)]
pub struct HealthRegen {
    pub rate: f32,
    pub cost: f32,
}

impl Default for HealthRegen {
    fn default() -> Self {
        Self {
            rate: 1.0,
            cost: 1.0,
        }
    }
}

pub fn regen_health(
    mut query: Query<(&HealthRegen, &Stamina, &Health, Entity)>,
    mut stamina: EventWriter<StaminaEvent>,
    mut health: EventWriter<HealthEvent>,
    time: Res<Time>,
) {
    for (regen, s, h, e) in query.iter_mut() {
        if s.is_starving() || h.is_full() {
            continue;
        }

        let delta = time.delta_seconds();
        health.send(HealthEvent::Heal(regen.rate * delta, e));
        stamina.send(StaminaEvent::Lose(regen.cost * delta, e));
    }
}

//TODO: Run crirera for when energy is 0
