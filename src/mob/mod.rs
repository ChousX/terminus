mod death;
pub mod debug;
mod diet;
mod energy;
mod health;
mod movement;

use crate::prelude::*;
use death::DeathEvent;
use energy::Stamina;
use health::HealthEvent;

use self::{
    death::death_handler,
    diet::{digestion, Stomach},
    energy::{stamina_event_handler, StaminaEvent},
    health::{health_event_handler, regen_health, Health, HealthRegen},
    movement::MovementPlugin,
};
use crate::select::Selectable;
pub use movement::{add_move_path_command, Pathing};
//NOTE: there should be ording for systems that use stamina
pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>()
            .add_system(death_handler)
            .add_event::<HealthEvent>()
            .add_system(health_event_handler)
            .add_system(regen_health)
            .add_system(digestion)
            .add_event::<StaminaEvent>()
            .add_system(stamina_event_handler)
            .add_startup_system(test_mob_2d)
            .add_plugin(MovementPlugin);
    }
}

#[derive(Bundle, Default)]
pub struct MobBundle {
    pub mob: Mob,
    pub stamina: Stamina,
    pub stomach: Stomach,
    pub health: Health,
    pub health_regeneration: HealthRegen,
    pub shape: ShapeBundle,
    pub selectable: Selectable,
}

impl MobBundle {
    pub fn new(pos: Vec2, sides: usize, size: f32) -> Self {
        let shape = shapes::RegularPolygon {
            sides,
            feature: shapes::RegularPolygonFeature::Radius(size),
            ..shapes::RegularPolygon::default()
        };

        let transform = Transform::from_translation(pos.extend(0.0));
        Self {
            mob: Mob { size, ..default() },
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform,
                ..default()
            },
            ..default()
        }
    }
}

#[derive(Component, Reflect)]
pub struct Mob {
    pub size: f32,
    pub shape: usize, //clan marking
    pub speed: f32,
    pub turn_rate: f32,
}

impl Default for Mob {
    fn default() -> Self {
        Self {
            size: 10.0,
            shape: 3,
            speed: 5.25,
            turn_rate: 1.0,
        }
    }
}

pub fn test_mob_2d(mut commands: Commands) {
    let size = 10.0;
    let space = 0.0;
    for x in 0..10 {
        for y in 0..10 {
            commands.spawn((
                MobBundle::new(
                    Vec2::new(x as f32 * size * 2. + space, y as f32 * size * 2. + space),
                    4,
                    size,
                ),
                Fill::color(Color::CYAN),
                Stroke::new(Color::BLACK, 0.2),
            ));
        }
    }
}
