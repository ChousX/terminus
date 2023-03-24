use crate::prelude::*;

use bevy::app::PluginGroupBuilder;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugins(pub bool);
impl PluginGroup for DebugPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut app = PluginGroupBuilder::start::<Self>();
        if !self.0 {
            return app;
        }
        #[cfg(debug_assertions)]
        {
            use crate::{
                camera::debug::CameraDebugPlugin, mob::debug::MobDebugPlugin,
                select::debug::SelectDebugPlugin,
            };

            app = app
                .add(WorldInspectorPlugin::new())
                .add(SelectDebugPlugin)
                .add(MobDebugPlugin)
                .add(CameraDebugPlugin);
        }
        app
    }
}
