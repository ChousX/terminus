pub mod keyboard;
pub mod mouse_keyboard;

use crate::prelude::*;

#[derive(Debug)]
pub enum Key {
    Mouse(MouseButton),
    Board(KeyCode),
}

pub type Mask = [bool; 3];

#[derive(Debug)]
pub struct Binding {
    pub keys: Vec<Key>,
    pub mask: Option<Mask>,
}

pub type ModKeys = [Vec<Key>; 3];

#[derive(Resource)]
pub struct ModBindings(pub ModKeys);
impl Default for ModBindings {
    fn default() -> Self {
        use KeyCode::*;
        Self([
            vec![Key::Board(LShift), Key::Board(RShift)],
            vec![Key::Board(LControl), Key::Board(RControl)],
            vec![Key::Board(LAlt), Key::Board(RAlt)],
        ])
    }
}

impl ModBindings {
    pub fn check(
        &self,
        keys: &Input<KeyCode>,
        mouse: &Input<MouseButton>,
        binding: &Binding,
    ) -> bool {
        if let Some(mask) = binding.mask {
            let modmask = get_mod_mask(keys, mouse, &self.0);
            if !check_mask(&modmask, &mask) {
                return false;
            }
        }
        check_keys(&binding.keys, keys, mouse)
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

fn check_mask(one: &Mask, two: &Mask) -> bool {
    for i in 0..(one.len()) {
        if one[i] != two[i] {
            return false;
        }
    }
    true
}

fn get_mod_mask(keys: &Input<KeyCode>, mouse: &Input<MouseButton>, mod_keys: &ModKeys) -> Mask {
    let mut out = [false; 3];
    for (i, key) in mod_keys.iter().enumerate() {
        out[i] = check_keys(key, keys, mouse);
    }
    out
}
