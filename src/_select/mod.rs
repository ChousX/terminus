pub mod debug;
mod keybord;
mod mouse;
mod selected;

use crate::input::*;
use crate::prelude::*;
use crate::utils::ModPressed;

use mouse::{in_univer_cursor_update, select, selection_handler};
use selected::path_selected;

#[derive(SystemSet, Clone, Eq, Debug, Hash, PartialEq)]
pub struct CursorGeneralUpdate;

#[derive(SystemSet, Clone, Eq, Debug, Hash, PartialEq)]
pub struct CursorPosUpdate;

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SelectionEvent>()
            .init_resource::<Selector>()
            .init_resource::<Selected>()
            .init_resource::<SelectorBindings>()
            .add_system(in_univer_cursor_update.run_if(is_mouse_and_keyboard))
            .add_system(
                select
                    .after(in_univer_cursor_update)
                    .run_if(is_mouse_and_keyboard),
            )
            .add_system(
                crate::mob::add_move_path_command
                    .run_if(path_selected)
                    .before(selection_handler),
            )
            .add_system(
                selection_handler
                    .run_if(on_event::<SelectionEvent>())
                    .after(in_univer_cursor_update),
            );
    }
}

#[derive(Resource, Default, InspectorOptions, Reflect)]
#[reflect(Resource, InspectorOptions)]
pub struct Selector {
    pub pos: Vec2,
    pub marker: Option<Vec2>,
}
#[derive(Resource)]
pub struct SelectorBindings {
    pub start_mark: Binding,
    pub path_to: Binding,
}

impl Default for SelectorBindings {
    fn default() -> Self {
        use Key::*;
        use MouseButton::*;
        Self {
            start_mark: vec![Mouse(Right), Board(KeyCode::M)],
            path_to: vec![Mouse(Left)],
        }
    }
}

impl SelectorBindings {
    pub fn get_mod_mask(&self) -> ModPressed {
        [true, false, false, false]
    }
}

pub enum SelectionEvent {
    Rectangle { p1: Vec2, p2: Vec2 },
    Single(Entity),
}

#[derive(Debug, Default, Resource, InspectorOptions, Reflect)]
#[reflect(Resource, InspectorOptions)]
pub struct Selected(pub Vec<Entity>);

#[derive(Component, Default)]
pub struct Selectable;
