use crate::{prelude::*, selection::Selector};

use super::{Binding, ModBindings};

#[derive(Resource)]
pub struct KeyboardBindings {
    move_up: Binding,
    move_down: Binding,
    move_left: Binding,
    move_right: Binding,
    move_selector_up: Binding,
    move_selector_down: Binding,
    move_selector_left: Binding,
    move_selector_right: Binding,
    start_selection: Binding,
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
            start_selection: Binding {
                keys: vec![Key::Board(Space)],
                mask: None,
            },
            move_selector_up: Binding {
                keys: vec![Key::Board(Up)],
                mask: None,
            },
            move_selector_down: Binding {
                keys: vec![Key::Board(Down)],
                mask: None,
            },
            move_selector_left: Binding {
                keys: vec![Key::Board(Left)],
                mask: None,
            },
            move_selector_right: Binding {
                keys: vec![Key::Board(Right)],
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

pub fn selector_start(
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    modkeys: Res<ModBindings>,
    bindings: Res<KeyboardBindings>,
) -> bool {
    modkeys.check(&keys, &mouse, &bindings.start_selection)
}

pub fn selector_end(
    selector: Res<Selector>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    modkeys: Res<ModBindings>,
    bindings: Res<KeyboardBindings>,
) -> bool {
    selector.marker.is_some() && !modkeys.check(&keys, &mouse, &bindings.start_selection)
}

use crate::selection::movement::SelectorMovementEvent as MoveSelector;

pub fn move_selector(
    mut out: EventWriter<MoveSelector>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    modkeys: Res<ModBindings>,
    bindings: Res<KeyboardBindings>,
) {
    if modkeys.check(&keys, &mouse, &bindings.move_selector_up) {
        out.send(MoveSelector::Up)
    }
    if modkeys.check(&keys, &mouse, &bindings.move_selector_down) {
        out.send(MoveSelector::Down)
    }
    if modkeys.check(&keys, &mouse, &bindings.move_selector_left) {
        out.send(MoveSelector::Left)
    }
    if modkeys.check(&keys, &mouse, &bindings.move_selector_right) {
        out.send(MoveSelector::Right)
    }
}
