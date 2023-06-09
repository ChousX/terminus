mod actions;
mod context;
mod desires;
mod percept;

use crate::prelude::*;
pub use percept::*;

use super::movement::MobMoveEvent;

pub struct CognitionPlugin;
impl Plugin for CognitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PerceptionEvent>()
            .add_systems(
                (
                    sight,
                    smell_ammet,
                    smell_combiner.after(smell_ammet),
                    sent.after(smell_combiner),
                )
                    .in_set(PerceptSet),
            )
            .add_system(PerceptionEvent::handle.after(PerceptSet));
    }
}

impl CognitionPlugin {
    fn _build(&self, app: &mut App) {
        // all
        app.add_event::<PerceptionEvent>();
        // Percept's
        app.add_systems((sent, sight, sound).in_set(PerceptSet));
        // Context
        app.add_system(context::build_context.after(PerceptSet));
        //Desires
        app.add_system(desires::inclination.after(context::build_context));
        //Actions
        app.add_system(MobMoveEvent::handle);
    }
}
