//this may be doomed to just a remake of Selector
mod actions;
mod bindings;
mod debug;
pub mod movement;
mod selector;

use crate::{prelude::*, selection::movement::SelectorMovementEvent};

pub use selector::Selector;

pub struct SelectorPlugin;
impl Plugin for SelectorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selector>()
            .init_resource::<Selected>()
            .add_event::<SelectorMovementEvent>()
            //        .add_startup_system(bindings::init)
            .add_system(actions::selector_mouce_syncer.before(SelectorMovementEvent::handle))
            .add_system(SelectorMovementEvent::handle.run_if(on_event::<SelectorMovementEvent>()))
            .add_systems(
                (
                    actions::cursor_selector_linker,
                    //                 actions::start_selection.run_if(bindings::start_selection),
                    //                   actions::stop_selection.run_if(bindings::stop_selection),
                    debug::dysplay_selection,
                )
                    .after(SelectorMovementEvent::handle),
            );
        info!("PluginLoaded");
    }
}

#[derive(Resource, Default, Deref)]
pub struct Selected(Vec<Entity>);

#[derive(Component, Default)]
pub struct Selectable;

#[derive(Component)]
pub struct Cursor;
