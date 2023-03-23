pub mod debug;
mod event;
mod input;
mod startup;

use crate::prelude::*;
use event::CameraEvent;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraEvent>()
            .init_resource::<input::CameraMoveKeyBindings>()
            .init_resource::<input::MouseSensitivity>()
            .init_resource::<input::MouseZoomSensitivity>()
            .add_startup_system(startup::new_camera)
            .add_startup_system(startup::test_seen_2d)
            .add_system(input::keybored)
            .add_system(input::mouse_move)
            .add_system(input::mouse_zoom)
            .add_system(event::camera_event_handler);
    }
}
