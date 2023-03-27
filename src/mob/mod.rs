mod actions; // (3) ware things are actuly done
mod context; // (1)The information that has been collected
mod desires; // (2)A list generated from the the context. Each entry is ranked by want, what ever is ranked heighest is preformed
mod movement;
mod perception; // (0)How the mob gets information

use crate::prelude::*;

use self::{context::Context, movement::MobMoveEvent, perception::PerceptSet};

pub struct MobPlugin;
impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        // all
        app.add_event::<perception::PerceptionEvent>()
            .add_event::<MobMoveEvent>();
        // Percept's
        app.add_systems(
            (perception::sent, perception::sight, perception::sound).in_set(PerceptSet),
        );
        // Context
        app.add_system(context::build_context.after(PerceptSet));
        //Desires
        app.add_system(desires::inclination.after(context::build_context));
        //Actions
        app.add_system(MobMoveEvent::handle);
    }
}

#[derive(Component)]
pub struct Mob;
