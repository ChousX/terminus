use crate::{mob::chunk::MobChunks, prelude::*};

use super::PerceptionEvent;

#[derive(Component, Debug)]
pub struct Olfactory {
    range: f32,
}

impl Default for Olfactory {
    fn default() -> Self {
        Self { range: 50.0 }
    }
}

#[derive(Debug, Component)]
pub struct Smell {
    range: f32,
    strangth: f32,
    timer: Timer,
}

impl Default for Smell {
    fn default() -> Self {
        Self {
            range: 50.0,
            strangth: 1.0,
            timer: Timer::from_seconds(60.0, TimerMode::Once),
        }
    }
}

impl Smell {
    pub fn make_check_box(&self, pos: Vec2) -> (Vec2, Vec2) {
        (
            Vec2::new(pos.x - self.range, pos.y - self.range),
            Vec2::new(pos.x + self.range, pos.y + self.range),
        )
    }
}

#[derive(Bundle, Default)]
pub struct SmellBundle {
    smell: Smell,
    pos: Transform,
}

#[derive(Debug, Component)]
pub struct SmellAmmeter {
    smell_templet: fn() -> Smell,
    rate: Timer,
}

/// Generates Smells at target location
pub fn smell_ammet(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut SmellAmmeter)>,
    time: Res<Time>,
) {
    for (transform, mut ammeter) in query.iter_mut() {
        if let Some(smell) = ammeter.update(&time) {
            commands.spawn(SmellBundle {
                pos: transform.clone(),
                ..default()
            });
        }
    }
}

impl SmellAmmeter {
    pub fn update(&mut self, time: &Res<Time>) -> Option<Smell> {
        let timer = self.rate.tick(time.delta());
        if timer.just_finished() {
            let func = self.smell_templet;
            let smell = func();
            Some(smell)
        } else {
            None
        }
    }
}

impl Default for SmellAmmeter {
    fn default() -> Self {
        Self {
            smell_templet: Smell::default,
            rate: Timer::from_seconds(10.0, TimerMode::Repeating),
        }
    }
}

pub fn sent(
    smells: Query<(&Smell, &Transform)>,
    mobs: Res<MobChunks>,
    query: Query<(&Transform, &Olfactory)>,
    mut out: EventWriter<PerceptionEvent>,
) {
    for (smell, smell_transform) in smells.iter() {
        let smell_pos = smell_transform.translation.truncate();
        let (one, two) = smell.make_check_box(smell_pos);
        for mob_e in mobs.get_in_range(one, two) {
            if let Ok((mob_transform, olfactory)) = query.get(mob_e) {
                let mob_pos = mob_transform.translation.truncate();
                let dif = (smell_pos - mob_pos).abs();
                let dif = (dif.x * dif.x + dif.y * dif.y).sqrt();
                if dif <= smell.range + olfactory.range {
                    out.send(PerceptionEvent::Sent(mob_e))
                }
            }
        }
    }
}

pub fn smell_combiner(mut query: Query<(&Smell, &Transform, Entity)>, mut commands: Commands) {
    //todo!()
}
