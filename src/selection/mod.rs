mod actions;
mod bindings;
mod debug;
mod selector;
use crate::prelude::*;
pub use selector::Selector;

pub struct SelectorPlugin;
impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app;
    }
}

#[derive(Resource, Default, Deref)]
pub struct Selected(Vec<Entity>);

#[derive(Component, Default)]
pub struct Selectable;

#[derive(Component)]
pub struct Cursor;
