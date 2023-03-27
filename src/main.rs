mod camera;
mod input;
mod mob;
mod prelude;
mod selection;

use crate::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::TURQUOISE))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(selection::SelectorPlugin)
        .run();
}
