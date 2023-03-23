use crate::prelude::*;
use bevy::utils::HashMap;

pub enum Key {
    Mouse(MouseButton),
    Board(KeyCode),
}

pub type Binding = Vec<Key>;

pub trait Bindable {
    fn get_all_bindings(&self) -> HashMap<String, KeyCode>;
}

pub struct KeyBindings {
    data: HashMap<String, KeyCode>,
}

impl KeyBindings {
    pub fn add<B: Bindable>(&mut self, settings: B) -> bool {
        let other = settings.get_all_bindings();
        if !self.overlapping(&other) {
            println!("woop");
            self.data.extend(other);
            true
        } else {
            false
        }
    }

    pub fn overlapping(&self, other: &HashMap<String, KeyCode>) -> bool {
        let map1 = &self.data;
        let map2 = other;

        map2.keys().all(|k| map1.contains_key(k))
    }
}

pub fn check(keys: &Input<KeyCode>, binding: &Binding, mouse: Option<&Input<MouseButton>>) -> bool {
    for key in binding.iter() {
        if let Key::Board(key) = key {
            if keys.pressed(*key) {
                return true;
            }
        }
    }

    if let Some(keys) = mouse {
        for key in binding.iter() {
            if let Key::Mouse(key) = key {
                if keys.pressed(*key) {
                    return true;
                }
            }
        }
    }

    false
}
pub const PRESSED_MOD: usize = 4;
pub type ModPressed = [bool; PRESSED_MOD];
#[derive(Resource)]
pub struct ModKeyBindings {
    pub one: Binding,
    pub two: Binding,
    pub three: Binding,
    pub four: Binding,
    pub pressed: ModPressed,
}

impl Default for ModKeyBindings {
    fn default() -> Self {
        use Key::*;
        use KeyCode::*;
        Self {
            one: vec![Board(LShift), Board(RShift)],
            two: vec![Board(LControl), Board(RControl)],
            three: vec![Board(LAlt), Board(RAlt)],
            four: vec![],
            pressed: [false; PRESSED_MOD],
        }
    }
}

impl ModKeyBindings {
    //could just use a bit mask of an u8
    pub fn active(&self, mask: &ModPressed) -> bool {
        for (i, b) in mask.iter().enumerate() {
            if *b {
                if !self.pressed[i] {
                    return false;
                }
            }
        }
        true
    }
}
