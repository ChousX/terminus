use bevy::input::mouse::MouseMotion;

use crate::prelude::*;

pub fn camera_zoom(
    mut camera: Query<&mut OrthographicProjection>,
    mut scroll_evr: EventReader<bevy::input::mouse::MouseWheel>,
) {
    let mut projection = camera.single_mut();
    // example: zoom in
    let mut total = 0.0;
    for scroll in scroll_evr.iter() {
        total += scroll.y;
    }
    projection.scale += total; //* time.delta_seconds();

    const MIN: f32 = 1.0;
    const MAX: f32 = 10.0;
    projection.scale = projection.scale.clamp(MIN, MAX);
}

pub fn move_left(mut event: EventWriter<CameraMoveEvent>) {
    event.send(CameraMoveEvent::Left)
}

pub fn move_right(mut event: EventWriter<CameraMoveEvent>) {
    event.send(CameraMoveEvent::Right)
}

pub fn move_up(mut event: EventWriter<CameraMoveEvent>) {
    event.send(CameraMoveEvent::Up)
}

pub fn move_down(mut event: EventWriter<CameraMoveEvent>) {
    event.send(CameraMoveEvent::Down)
}

pub fn camera_move_mouce(
    mut camera_move: EventWriter<CameraMoveEvent>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let mut translation = Vec2::ZERO;
    for mouse_motion in motion_evr.iter() {
        translation += mouse_motion.delta;
    }
    camera_move.send(CameraMoveEvent::Amount(translation));
}
