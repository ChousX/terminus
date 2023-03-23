use crate::prelude::*;

#[derive(Default, Resource, PartialEq)]
pub enum InputState {
    #[default]
    MouseKeybored,
    Keybored,
    Controler,
}
impl InputState {
    pub fn is_mouse_and_keyboard(&self) -> bool {
        *self == Self::MouseKeybored
    }
    pub fn is_keyboard(&self) -> bool {
        *self == Self::Keybored
    }
    pub fn is_controler(&self) -> bool {
        *self == Self::Controler
    }
}

pub fn is_mouse_and_keyboard(state: Res<InputState>) -> bool {
    state.is_mouse_and_keyboard()
}

pub fn is_keyboard(state: Res<InputState>) -> bool {
    state.is_keyboard()
}

pub fn is_controler(state: Res<InputState>) -> bool {
    state.is_controler()
}
