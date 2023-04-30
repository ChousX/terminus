use crate::prelude::*;
use bird_binding::{Binding, Bindings, Key, UserInput};
use bird_binding_derive::{BindFoos, BirdBinding};

#[derive(BirdBinding, BindFoos)]
pub struct SelectionBindings {
    selection_active: Binding,
}

impl Default for SelectionBindings {
    fn default() -> Self {
        SelectionBindings {
            selection_active: Binding::Pressed(vec![Key::Mouse(MouseButton::Left)]),
        }
    }
}
