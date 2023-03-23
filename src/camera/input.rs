use bevy::input::mouse::{MouseMotion, MouseWheel};

use crate::prelude::*;

use crate::utils::check;

use super::event::CameraEvent;

#[derive(Resource)]
pub struct CameraMoveKeyBindings {
    pub up: Binding,
    pub down: Binding,
    pub left: Binding,
    pub right: Binding,
    pub sensitivity: f32,
}

impl Default for CameraMoveKeyBindings {
    fn default() -> Self {
        use Key::*;
        use KeyCode::*;
        Self {
            up: vec![Board(W), Board(Up)],
            down: vec![Board(S), Board(Down)],
            left: vec![Board(A), Board(Left)],
            right: vec![Board(D), Board(Right)],
            sensitivity: 1.0,
        }
    }
}

pub fn keybored(
    bind: Res<CameraMoveKeyBindings>,
    keys: Res<Input<KeyCode>>,
    mut output: EventWriter<CameraEvent>,
) {
    let mut amount = Vec3::ZERO;
    if check(&keys, &bind.up, None) {
        amount.y += bind.sensitivity;
    }
    if check(&keys, &bind.down, None) {
        amount.y -= bind.sensitivity;
    }
    if check(&keys, &bind.left, None) {
        amount.x -= bind.sensitivity;
    }
    if check(&keys, &bind.right, None) {
        amount.x += bind.sensitivity;
    }
    if amount != Vec3::ZERO {
        output.send(CameraEvent::MoveBy(amount));
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct MouseZoomSensitivity(f32);
impl Default for MouseZoomSensitivity {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct MouseSensitivity(f32);
impl Default for MouseSensitivity {
    fn default() -> Self {
        Self(1.0)
    }
}

pub fn mouse_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    sensitivity: Res<MouseZoomSensitivity>,
    mut output: EventWriter<CameraEvent>,
) {
    let mut amount = 0f32;
    for event in scroll_evr.iter() {
        amount += event.x * sensitivity.0;
        amount += event.y * sensitivity.0;
    }
    if amount != 0f32 {
        output.send(CameraEvent::Zoom(amount));
    }
}

pub fn mouse_move(
    mut motion_evr: EventReader<MouseMotion>,
    sensitivity: Res<MouseSensitivity>,
    mut output: EventWriter<CameraEvent>,
) {
}
