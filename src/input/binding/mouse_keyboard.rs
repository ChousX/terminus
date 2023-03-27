use bevy::input::mouse::MouseMotion;

use super::{Binding, ModBindings};
use crate::{prelude::*, selection::Selector};

#[derive(Resource)]
pub struct MouseKeyboardBindings {
    move_up: Binding,
    move_down: Binding,
    move_left: Binding,
    move_right: Binding,

    mouse_move: Binding,
    start_selection: Binding,
}

impl Default for MouseKeyboardBindings {
    fn default() -> Self {
        use KeyCode::*;
        Self {
            move_up: Binding {
                keys: vec![Key::Board(W), Key::Board(Up)],
                mask: None,
            },
            move_down: Binding {
                keys: vec![Key::Board(S), Key::Board(Down)],
                mask: None,
            },
            move_right: Binding {
                keys: vec![Key::Board(A), Key::Board(Left)],
                mask: None,
            },
            move_left: Binding {
                keys: vec![Key::Board(D), Key::Board(Right)],
                mask: None,
            },
            mouse_move: Binding {
                keys: vec![Key::Mouse(MouseButton::Right)],
                mask: None,
            },
            start_selection: Binding {
                keys: vec![Key::Mouse(MouseButton::Left)],
                mask: None,
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

pub fn selector_start(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    modkeys: Res<ModBindings>,
    bindings: Res<MouseKeyboardBindings>,
) -> bool {
    modkeys.check(&keys, &mouse, &bindings.start_selection)
}

pub fn selector_end(
    selector: Res<Selector>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    modkeys: Res<ModBindings>,
    bindings: Res<MouseKeyboardBindings>,
) -> bool {
    selector.marker.is_some() && !modkeys.check(&keys, &mouse, &bindings.start_selection)
}
