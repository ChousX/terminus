use crate::prelude::*;
use bird_binding::{Binding, Key, UserInput};
use bird_binding_derive::{BindFoos, BirdBinding};

use super::Selector;

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

pub fn not_selection_active(input: Res<UserInput>, selection: Res<Selector>) -> bool {
    !input.check("selection_active") && selection.marker.is_some()
}
