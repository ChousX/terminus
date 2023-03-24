use crate::prelude::*;

use super::{Binding, ModBindings, ModKeys};

#[derive(Resource)]
pub struct KeyboardBindings {
    move_up: Binding,
    move_down: Binding,
    move_left: Binding,
    move_right: Binding,
}

impl Default for KeyboardBindings {
    fn default() -> Self {
        use KeyCode::*;
        Self {
            move_up: Binding {
                keys: vec![Key::Board(W)],
                mask: None,
            },
            move_down: Binding {
                keys: vec![Key::Board(S)],
                mask: None,
            },
            move_right: Binding {
                keys: vec![Key::Board(A)],
                mask: None,
            },
            move_left: Binding {
                keys: vec![Key::Board(D)],
                mask: None,
            },
        }
    }
}

pub fn camera_move(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    bindings: Res<KeyboardBindings>,
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
