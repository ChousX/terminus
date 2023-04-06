use crate::prelude::*;

use super::stamina::Stamina;

pub enum Food {
    Plant(f32),
    Animal(f32),
    //I would like to have minrules too
}

impl Food {
    pub fn ammount(&self) -> f32 {
        match *self {
            Food::Animal(ammount) => ammount,
            Food::Plant(ammount) => ammount,
        }
    }
}

pub trait Edible {
    fn to_food(&self) -> Food;
}

#[derive(Component)]
pub struct Stomach {
    food: Vec<Food>,
    max_ammount: f32,
    current_ammount: f32,
    rate: f32,
    efect: [f32; 2],
}

impl Default for Stomach {
    fn default() -> Self {
        Self {
            food: default(),
            max_ammount: 100.0,
            current_ammount: 0.0,
            rate: 1.0,
            efect: [1.0, 1.0],
        }
    }
}

impl Stomach {
    pub fn add(&mut self, food: impl Edible) -> bool {
        let food = food.to_food();
        let add_amount = self.current_ammount + food.ammount();
        if add_amount > self.max_ammount {
            return false;
        }
        self.current_ammount = add_amount;
        self.food.push(food);
        true
    }

    ///returns energy and waist ammounts
    pub fn digest(&mut self, delta_sec: f32) -> (f32, f32) {
        let mut remove_list = Vec::new();
        let mut acum_energy = 0.0f32;
        let mut acum_waist = 0.0f32;
        for (i, food) in self.food.iter_mut().enumerate() {
            acum_energy += match food {
                Food::Animal(ammount) => {
                    let sub = self.rate * delta_sec;
                    *ammount -= sub;
                    let ammount_of_food = if *ammount <= 0.0 {
                        remove_list.push(i);
                        sub - ammount.abs()
                    } else {
                        sub
                    };
                    acum_waist += ammount_of_food;
                    ammount_of_food * self.efect[0]
                }
                Food::Plant(ammount) => {
                    let sub = self.rate * delta_sec;
                    *ammount -= sub;
                    let ammount_of_food = if *ammount <= 0.0 {
                        remove_list.push(i);
                        sub - ammount.abs()
                    } else {
                        sub
                    };
                    acum_waist += ammount_of_food;
                    ammount_of_food * self.efect[1]
                }
            }
        }
        //running through the remove_list form last added to first
        // and removing them from self.food
        for i in remove_list.into_iter().rev() {
            self.food.remove(i);
        }

        (acum_energy, acum_waist)
    }
}

pub fn digestion(mut query: Query<(&mut Stomach, &mut Stamina)>, time: Res<Time>) {
    for (mut stomach, mut stamina) in query.iter_mut() {
        let (energy, _waist) = stomach.digest(time.delta_seconds());
        stamina.add(energy);
    }
}
