mod chunk;
pub mod cognition;
mod digestion;
mod health;
mod movement;
mod pathing;
mod stamina;

use crate::{prelude::*, selection::Selectable};

use bevy_prototype_lyon::prelude::{shapes, Fill, GeometryBuilder, ShapeBundle, Stroke};
use cognition::CognitionPlugin;

use self::{
    chunk::MobChunks,
    cognition::PerceptionBundle,
    digestion::{digestion, Stomach},
    movement::MobMoveData,
    stamina::Stamina,
};

pub use movement::MobMoveEvent;
pub use pathing::{add_pos_to_selected_mobs, new_path_for_selected_mobs};

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MobChunks>()
            .add_system(MobChunks::update)
            .add_event::<movement::MobMoveEvent>()
            .add_system(movement::MobMoveEvent::handle)
            .add_system(pathing::path_consumer.before(MobMoveEvent::handle))
            //.add_system(movement::test.before(movement::MobMoveEvent::handle))
            .add_startup_system(debug::test_mob_2d)
            .add_system(chunk::debug::dysplay_boxes.after(MobChunks::update))
            .add_system(digestion)
            .add_system(debug::mod_dir_line)
            .add_plugin(CognitionPlugin);
    }
}

#[derive(Component, Default)]
pub struct Mob;

#[derive(Bundle, Default)]
pub struct MobBundle {
    pub mob: Mob,
    pub selectable: Selectable,
    pub shape: ShapeBundle,
    pub stamina: Stamina,
    pub stomach: Stomach,
    #[bundle]
    pub perseption: PerceptionBundle,
    pub mob_move_data: MobMoveData,
}

impl MobBundle {
    pub fn new(pos: Vec2) -> Self {
        let shape = shapes::RegularPolygon {
            sides: 3,
            feature: shapes::RegularPolygonFeature::Radius(10.0),
            ..shapes::RegularPolygon::default()
        };
        let transform = Transform::from_translation(pos.extend(0.0));
        Self {
            shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                transform,
                ..default()
            },
            ..default()
        }
    }
}

mod debug {
    use super::*;
    use bevy::prelude::*;
    use bevy_prototype_debug_lines::DebugLines;

    pub fn mod_dir_line(
        transforms: Query<(&Transform, &Mob), With<Mob>>,
        mut lines: ResMut<DebugLines>,
    ) {
        for (transform, mob) in transforms.iter() {
            let start = transform.translation;
            let forward_normal = (transform.rotation * Vec3::Y).truncate();
            let end = (Vec2::splat(10.0 + 5.) * forward_normal).extend(0.0) + start;
            lines.line(start, end, 0.0)
        }
    }

    pub fn test_mob_2d(mut commands: Commands) {
        for x in 0..10 {
            for y in 0..10 {
                commands.spawn((
                    MobBundle::new(Vec2::new(
                        x as f32 * 10.0 * 2. + 1.0,
                        y as f32 * 10.0 * 2. + 1.0,
                    )),
                    Fill::color(Color::CYAN),
                    Stroke::new(Color::BLACK, 0.2),
                ));
            }
        }
    }
}
