mod keyboard;
mod mouse_keyboard;

use crate::{camera::CameraMoveEvent, prelude::*};

#[derive(Debug)]
pub enum Key {
    Mouse(MouseButton),
    Board(KeyCode),
}

pub type Mask = [bool; 4];

#[derive(Debug)]
pub struct Binding {
    keys: Vec<Key>,
    mask: Mask,
}

pub type ModKeys = [Key; 4];

impl Binding {
    pub fn check(
        &self,
        keys: &Input<KeyCode>,
        mouse: &Input<MouseButton>,
        modkeys: &ModKeys,
    ) -> bool {
        let mod_mask = get_mod_mask(keys, mouse, modkeys);
        check_mask(self.mask, mod_mask) && check_keys(&self.keys, keys, mouse)
    }
}

fn check_keys(bindings: &[Key], keys: &Input<KeyCode>, mouse: &Input<MouseButton>) -> bool {
    for binding in bindings.iter() {
        match *binding {
            Key::Board(key) => {
                if keys.pressed(key) {
                    return true;
                }
            }
            Key::Mouse(key) => {
                if mouse.pressed(key) {
                    return true;
                }
            }
        }
    }
    false
}

fn check_mask(one: Mask, two: Mask) -> bool {
    for i in 0..(one.len()) {
        if one[i] != two[i] {
            return false;
        }
    }
    true
}

fn get_mod_mask(keys: &Input<KeyCode>, mouse: &Input<MouseButton>, mod_keys: &ModKeys) -> Mask {
    let mut out = [false, false, false, false];
    for (i, key) in mod_keys.iter().enumerate() {
        if match *key {
            Key::Board(key) => keys.pressed(key),
            Key::Mouse(key) => mouse.pressed(key),
        } {
            out[i] = true;
        }
    }
    out
}

pub trait Bindable {
    fn move_camera(
        &self,
        keys: &Input<KeyCode>,
        mouse: &Input<MouseButton>,
    ) -> Option<CameraMoveEvent>;
    fn start_selection(&self, keys: &Input<KeyCode>, mouse: &Input<MouseButton>) -> bool;
    fn end_selection(&self, keys: &Input<KeyCode>, mouse: &Input<MouseButton>) -> bool;
}
