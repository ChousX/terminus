mod camera;
mod chunk;
mod map;
mod mob;
mod prelude;
mod selection;

use crate::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::TURQUOISE))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(bird_binding::BirdBindingPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(selection::SelectorPlugin)
        .add_plugin(mob::MobPlugin)
        .run();
}
