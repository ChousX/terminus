use crate::prelude::*;

use super::PerceptionEvent;

pub struct SoundEvent {
    strength: f32,
    sound_kind: Sound,
    pos: Vec2,
}

pub enum Sound {
    Unknone,
}

#[derive(Component, Debug)]
pub struct Audible {
    loudness: f32,
}

impl Audible {
    pub fn generate(&self, power: f32) -> f32 {
        self.loudness * power
    }
}

impl Default for Audible {
    fn default() -> Self {
        Self { loudness: 1.0 }
    }
}

pub fn sound(out: EventWriter<PerceptionEvent>) {
    todo!()
}

fn in_circle(center: &Vec2, radias: f32, check: &Vec2) -> bool {
    todo!()
}
