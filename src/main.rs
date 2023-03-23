mod camera;
mod debug;
mod input;
mod mob;
mod prelude;
mod select;
mod utils;

use crate::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::TURQUOISE))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(ShapePlugin)
        .init_resource::<input::InputState>()
        .init_resource::<ModKeyBindings>()
        .add_plugin(camera::CameraPlugin)
        .add_plugin(mob::MobPlugin)
        .add_plugin(select::SelectionPlugin)
        .add_plugins(debug::DebugPlugins(true))
        .run();
}
