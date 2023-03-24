use super::*;
use crate::prelude::*;

pub fn path_selected(
    selector_bindings: Res<SelectorBindings>,
    mod_bindings: Res<ModKeyBindings>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
) -> bool {
    let mod_mask = selector_bindings.get_mod_mask();
    let active = check(&keys, &selector_bindings.path_to, Some(&buttons));
    active && mod_bindings.active(&mod_mask)
}
