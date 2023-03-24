use bevy::input::mouse::MouseMotion;

use super::{Binding, ModBindings};
use crate::prelude::*;

#[derive(Resource)]
pub struct MouseKeyboardBindings {
    move_up: Binding,
    move_down: Binding,
    move_left: Binding,
    move_right: Binding,

    mouse_move: Binding,
}

impl Default for MouseKeyboardBindings {
    fn default() -> Self {
        use KeyCode::*;
        Self {
            move_up: Binding {
                keys: vec![Key::Board(W), Key::Board(Up)],
                mask: Some([false; 3]),
            },
            move_down: Binding {
                keys: vec![Key::Board(S), Key::Board(Down)],
                mask: Some([false; 3]),
            },
            move_right: Binding {
                keys: vec![Key::Board(A), Key::Board(Right)],
                mask: Some([false; 3]),
            },
            move_left: Binding {
                keys: vec![Key::Board(D), Key::Board(Left)],
                mask: Some([false; 3]),
            },
            mouse_move: Binding {
                keys: vec![Key::Mouse(MouseButton::Right)],
                mask: Some([false; 3]),
            },
        }
    }
}

pub fn camera_move_keys(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    bindings: Res<MouseKeyboardBindings>,
    modkeys: Res<ModBindings>,
    mut camera_move: EventWriter<CameraMoveEvent>,
) {
    if modkeys.check(&keys, &mouse, &bindings.move_up) {
        camera_move.send(CameraMoveEvent::Up)
    }
    if modkeys.check(&keys, &mouse, &bindings.move_down) {
        camera_move.send(CameraMoveEvent::Down)
    }
    if modkeys.check(&keys, &mouse, &bindings.move_left) {
        camera_move.send(CameraMoveEvent::Left)
    }
    if modkeys.check(&keys, &mouse, &bindings.move_right) {
        camera_move.send(CameraMoveEvent::Right)
    }
}

pub fn camera_move_mouce(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    bindings: Res<MouseKeyboardBindings>,
    modkeys: Res<ModBindings>,
    mut camera_move: EventWriter<CameraMoveEvent>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    if modkeys.check(&keys, &mouse, &bindings.mouse_move) {
        let mut translation = Vec2::ZERO;
        for mouse_motion in motion_evr.iter() {
            translation += mouse_motion.delta;
        }
        camera_move.send(CameraMoveEvent::Amount(translation));
    }
}
