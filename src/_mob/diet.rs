use super::{Stamina, StaminaEvent};
use crate::prelude::*;

#[derive(Sequence)]
pub enum DietType {
    Carnavor,
    Omnivore,
    Herbivore,
}

#[derive(Reflect)]
pub struct Diet {
    food: Vec<(Food, f32)>,
}

impl Default for Diet {
    fn default() -> Self {
        DietType::Omnivore.into()
    }
}

impl Diet {
    fn can_eat(&self, food: Food) -> Option<f32> {
        if let Some((_, rate)) = self.food.iter().find(|(diet_food, _)| food == *diet_food) {
            Some(*rate)
        } else {
            None
        }
    }
}

impl From<DietType> for Diet {
    fn from(value: DietType) -> Self {
        use DietType::*;
        use Food::*;
        let food = match value {
            Carnavor => vec![(Meat, 1.0)],
            Omnivore => vec![(Meat, 0.5), (Plant, 0.5)],
            Herbivore => vec![(Plant, 1.0)],
        };
        Self { food }
    }
}

#[derive(Sequence, PartialEq, Eq, Copy, Clone, Reflect, FromReflect)]
pub enum Food {
    Meat,
    Plant,
}

#[derive(Component, Reflect)]
pub struct Stomach {
    pub digest: Vec<(Food, f32)>,
    pub diet: Diet,
    pub size: usize,
}

impl Default for Stomach {
    fn default() -> Self {
        Self::new(Diet::default(), 15)
    }
}

impl Stomach {
    pub fn new(diet: Diet, size: usize) -> Self {
        let digest = Vec::with_capacity(size);
        Self { digest, diet, size }
    }

    pub fn eat(&mut self, item: (Food, f32)) -> bool {
        if self.is_full() {
            return false;
        }

        let (food, unit) = item;
        if let Some(_) = self.diet.can_eat(food) {
            self.digest.push((food, unit));
            true
        } else {
            false
        }
    }

    pub fn is_full(&self) -> bool {
        self.digest.len() >= self.size
    }

    pub fn digest(&mut self, time_delta: f32) -> f32 {
        let mut energy = 0.0;
        let mut remove_list = Vec::new();
        for (i, (food, unit)) in self.digest.iter_mut().enumerate() {
            let val = if let Some(rate) = self.diet.can_eat(*food) {
                rate * *unit * time_delta
            } else {
                info!("Eat food they can't eat some how...");
                *unit
            };
            *unit -= val;
            energy += val;
            if *unit < f32::EPSILON {
                remove_list.push(i);
            }
        }
        for i in remove_list.into_iter().rev() {
            self.digest.remove(i);
        }
        energy
    }
}

pub fn digestion(
    mut query: Query<(&mut Stomach, Entity), With<Stamina>>,
    time: Res<Time>,
    mut out: EventWriter<StaminaEvent>,
) {
    for (mut stomach, e) in query.iter_mut() {
        let added_energy = stomach.digest(time.delta_seconds());
        out.send(StaminaEvent::Gain(added_energy, e))
    }
}

pub fn hungry() {}
