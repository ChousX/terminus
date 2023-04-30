use bird_binding_derive::{BindFoos, BirdBinding};

use crate::prelude::*;

#[derive(BirdBinding, BindFoos)]
pub struct CameraBindings {
    scrole_by_mouce_wheel: Binding,
    move_by_mouce_motion: Binding,

    zoom_in: Binding,
    zoom_out: Binding,

    camera_move_right: Binding,
    camera_move_left: Binding,
    camera_move_up: Binding,
    camera_move_down: Binding,
}

impl Default for CameraBindings {
    fn default() -> Self {
        use bird_binding::key_codes::*;
        Self {
            scrole_by_mouce_wheel: bind!(Key::Board(M)),
            move_by_mouce_motion: bind!(Key::Mouse(MLeft), Key::Board(LShift)),

            zoom_in: bind!(Key::Board(Equals)),
            zoom_out: bind!(Key::Board(Minus)),

            camera_move_right: bind!(Key::Board(Right)),
            camera_move_left: bind!(Key::Board(Left)),
            camera_move_up: bind!(Key::Board(Up)),
            camera_move_down: bind!(Key::Board(Down)),
        }
    }
}
