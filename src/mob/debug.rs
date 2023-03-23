use crate::prelude::*;
use bevy_prototype_debug_lines::*;

use super::{
    diet::Stomach,
    energy::Stamina,
    health::{Health, HealthRegen},
    Mob,
};

pub struct MobDebugPlugin;
impl Plugin for MobDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_inspector_egui::quick::FilterQueryInspectorPlugin::<
            With<Mob>,
        >::default());

        app.register_type::<Health>().register_type::<HealthRegen>();

        app.register_type::<Stamina>();

        app.register_type::<Stomach>();

        app.add_system(mod_dir_line);

        app.register_type::<Mob>();
    }
}

pub fn mod_dir_line(
    transforms: Query<(&Transform, &Mob), With<Mob>>,
    mut lines: ResMut<DebugLines>,
) {
    for (transform, mob) in transforms.iter() {
        let start = transform.translation;
        let forward_normal = (transform.rotation * Vec3::Y).truncate();
        let end = (Vec2::splat(mob.size + 5.) * forward_normal).extend(0.0) + start;
        lines.line(start, end, 0.0)
    }
}
